use crate::{
    config::{Stream, StreamConfig},
    db::{self, ControllerTipProofRow, EventAppendedRow, ResolvedStream},
    decode, domain, reorg,
    rpc::RpcProviders,
    util,
};
use alloy::{
    primitives::{Address, U256},
    providers::Provider,
    rpc::types::Filter,
    sol_types::SolEvent,
};
use anyhow::{Context, Result};
use backoff::ExponentialBackoffBuilder;
use backoff::backoff::Backoff;
use futures::{StreamExt, stream};
use lru::LruCache;
use std::{collections::HashSet, future::Future, num::NonZeroUsize, sync::Arc, time::Duration};
use tokio::{sync::Semaphore, time};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};
use untron_v3_bindings::{
    untron_controller_index::UntronControllerIndex, untron_v3_index::UntronV3Index,
};

pub async fn run_stream(
    dbh: db::Db,
    cfg: StreamConfig,
    resolved: ResolvedStream,
    providers: RpcProviders,
    shutdown: CancellationToken,
    block_header_concurrency: usize,
    block_timestamp_cache_size: usize,
) -> Result<()> {
    let (stream, chain_id, contract_address_db) = resolved.into_parts();
    let contract_address_rpc = resolve_rpc_contract_address(stream, contract_address_db.as_str())?;

    let mut from_block = db::resume_from_block(&dbh, stream, cfg.deployment_block).await?;
    info!(
        stream = stream.as_str(),
        chain_id,
        contract_db = %contract_address_db,
        contract_rpc = %contract_address_rpc,
        from_block,
        "stream starting"
    );

    let mut state = PollState {
        stream,
        chain_id: i64::try_from(chain_id).context("chain_id out of range for bigint")?,
        contract_address_db,
        contract_address_rpc,
        confirmations: cfg.confirmations,
        reorg_scan_depth: cfg.reorg_scan_depth,
        chunk_target: cfg.chunk_blocks.max(1),
        chunk_current: cfg.chunk_blocks.max(1),
        pinned_providers: providers.pinned,
        provider: providers.fallback,
        timestamp_cache: BlockTimestampCache::new(block_timestamp_cache_size),
        header_sem: Arc::new(Semaphore::new(block_header_concurrency.max(1))),
        header_concurrency: block_header_concurrency.max(1),
    };

    let mut ticker = time::interval(cfg.poll_interval.max(Duration::from_secs(1)));
    ticker.set_missed_tick_behavior(time::MissedTickBehavior::Delay);

    let mut transient_attempts: u32 = 0;
    let mut transient_backoff = ExponentialBackoffBuilder::new()
        .with_initial_interval(Duration::from_millis(250))
        .with_max_interval(Duration::from_secs(2))
        .build();

    loop {
        tokio::select! {
            _ = shutdown.cancelled() => {
                info!(stream = state.stream.as_str(), "shutdown signal received");
                return Ok(());
            }
            _ = ticker.tick() => {}
        }

        let Some(head) = await_or_cancel(&shutdown, async {
            state
                .provider
                .get_block_number()
                .await
                .context("eth_blockNumber")
        })
        .await?
        else {
            return Ok(());
        };
        let head: u64 = head;

        let safe_head = head.saturating_sub(state.confirmations);
        debug!(
            stream = state.stream.as_str(),
            head, safe_head, from_block, "tick"
        );

        if let Some(reorg_start) = await_or_cancel(&shutdown, async {
            reorg::detect_reorg_start(
                &dbh,
                &state.provider,
                &state.pinned_providers,
                state.stream,
                state.reorg_scan_depth,
            )
            .await
        })
        .await?
        .flatten()
        {
            warn!(
                stream = state.stream.as_str(),
                reorg_start, "reorg detected; invalidating"
            );

            if await_or_cancel(
                &shutdown,
                db::invalidate_from_block(&dbh, state.stream, reorg_start),
            )
            .await?
            .is_none()
            {
                return Ok(());
            }
            state.timestamp_cache.clear();
            from_block = from_block.min(reorg_start);
        }

        while from_block <= safe_head {
            if shutdown.is_cancelled() {
                return Ok(());
            }

            let to_block = safe_head.min(from_block.saturating_add(state.chunk_current - 1));

            match process_range(&dbh, &shutdown, &mut state, from_block, to_block).await {
                Ok(()) => {
                    from_block = to_block.saturating_add(1);
                    transient_attempts = 0;
                    transient_backoff.reset();
                    if state.chunk_current < state.chunk_target {
                        state.chunk_current =
                            (state.chunk_current.saturating_mul(2)).min(state.chunk_target);
                    }
                }
                Err(e) => {
                    if looks_like_transient(&e) && transient_attempts < 3 {
                        transient_attempts += 1;
                        warn!(
                            stream = state.stream.as_str(),
                            from_block,
                            to_block,
                            attempt = transient_attempts,
                            err = %e,
                            "transient RPC error; retrying range"
                        );
                        let backoff = transient_backoff
                            .next_backoff()
                            .unwrap_or(Duration::from_millis(250));
                        tokio::select! {
                            _ = shutdown.cancelled() => return Ok(()),
                            _ = time::sleep(backoff) => {}
                        }
                        continue;
                    }

                    if state.chunk_current > 1 && looks_like_range_too_large(&e) {
                        transient_attempts = 0;
                        transient_backoff.reset();
                        state.chunk_current = (state.chunk_current / 2).max(1);
                        warn!(
                            stream = state.stream.as_str(),
                            from_block,
                            to_block,
                            chunk_blocks = state.chunk_current,
                            err = %e,
                            "eth_getLogs failed; shrinking chunk"
                        );
                        continue;
                    }

                    if state.chunk_current > 1 {
                        transient_attempts = 0;
                        transient_backoff.reset();
                        state.chunk_current = (state.chunk_current / 2).max(1);
                        warn!(
                            stream = state.stream.as_str(),
                            from_block,
                            to_block,
                            chunk_blocks = state.chunk_current,
                            err = %e,
                            "range processing failed; shrinking chunk"
                        );
                        continue;
                    }

                    // chunk_current == 1: try each pinned provider for this block
                    if !state.pinned_providers.is_empty() {
                        transient_attempts = 0;
                        transient_backoff.reset();
                        warn!(
                            stream = state.stream.as_str(),
                            block = from_block,
                            err = %e,
                            "range processing failed; attempting pinned providers"
                        );

                        let mut repaired = false;
                        for idx in 0..state.pinned_providers.len() {
                            if shutdown.is_cancelled() {
                                return Ok(());
                            }
                            let pinned = state.pinned_providers[idx].clone();
                            match process_range_with_provider(
                                &dbh, &shutdown, &mut state, &pinned, from_block, to_block,
                            )
                            .await
                            {
                                Ok(()) => {
                                    info!(
                                        stream = state.stream.as_str(),
                                        block = from_block,
                                        pinned_index = idx,
                                        "repair succeeded"
                                    );
                                    repaired = true;
                                    break;
                                }
                                Err(e2) => {
                                    warn!(
                                        stream = state.stream.as_str(),
                                        block = from_block,
                                        pinned_index = idx,
                                        err = %e2,
                                        "pinned provider failed"
                                    );
                                }
                            }
                        }

                        if repaired {
                            from_block = to_block.saturating_add(1);
                            continue;
                        }
                    }

                    error!(
                        stream = state.stream.as_str(),
                        from_block,
                        to_block,
                        err = %e,
                        "range processing failed permanently"
                    );
                    return Err(e);
                }
            }
        }
    }
}

#[derive(Clone)]
struct PollState {
    stream: Stream,
    chain_id: i64,
    contract_address_db: domain::ContractAddressDb,
    contract_address_rpc: Address,

    confirmations: u64,
    reorg_scan_depth: u64,

    chunk_target: u64,
    chunk_current: u64,

    provider: alloy::providers::DynProvider,
    pinned_providers: Vec<alloy::providers::DynProvider>,

    timestamp_cache: BlockTimestampCache,
    header_sem: Arc<Semaphore>,
    header_concurrency: usize,
}

#[derive(Clone)]
struct BlockTimestampCache {
    inner: LruCache<u64, u64>,
}

impl BlockTimestampCache {
    fn new(capacity: usize) -> Self {
        let cap = NonZeroUsize::new(capacity.max(1)).expect("nonzero");
        Self {
            inner: LruCache::new(cap),
        }
    }

    fn clear(&mut self) {
        self.inner.clear();
    }

    fn get(&mut self, block_number: u64) -> Option<u64> {
        self.inner.get(&block_number).copied()
    }

    fn peek(&self, block_number: u64) -> Option<u64> {
        self.inner.peek(&block_number).copied()
    }

    fn insert(&mut self, block_number: u64, timestamp: u64) {
        self.inner.put(block_number, timestamp);
    }
}

async fn process_range(
    dbh: &db::Db,
    shutdown: &CancellationToken,
    state: &mut PollState,
    from_block: u64,
    to_block: u64,
) -> Result<()> {
    let provider = state.provider.clone();
    process_range_with_provider(dbh, shutdown, state, &provider, from_block, to_block).await
}

async fn process_range_with_provider(
    dbh: &db::Db,
    shutdown: &CancellationToken,
    state: &mut PollState,
    provider: &alloy::providers::DynProvider,
    from_block: u64,
    to_block: u64,
) -> Result<()> {
    let event_appended_topic0 = match state.stream {
        Stream::Hub => UntronV3Index::EventAppended::SIGNATURE_HASH,
        Stream::Controller => UntronControllerIndex::EventAppended::SIGNATURE_HASH,
    };

    let filter = Filter::new()
        .address(state.contract_address_rpc)
        .from_block(from_block)
        .to_block(to_block)
        .event_signature(event_appended_topic0);

    let event_logs = await_or_cancel(shutdown, async {
        provider
            .get_logs(&filter)
            .await
            .with_context(|| format!("eth_getLogs EventAppended [{from_block}..{to_block}]"))
    })
    .await?
    .unwrap_or_default();
    if shutdown.is_cancelled() {
        return Ok(());
    }

    let mut proof_logs = Vec::new();
    if state.stream == Stream::Controller {
        let proof_filter = Filter::new()
            .address(state.contract_address_rpc)
            .from_block(from_block)
            .to_block(to_block)
            .event_signature(UntronControllerIndex::IsEventChainTipCalled::SIGNATURE_HASH);
        proof_logs = await_or_cancel(shutdown, async {
            provider.get_logs(&proof_filter).await.with_context(|| {
                format!("eth_getLogs IsEventChainTipCalled [{from_block}..{to_block}]")
            })
        })
        .await?
        .unwrap_or_default();
        if shutdown.is_cancelled() {
            return Ok(());
        }
    }

    let mut event_logs = validate_logs(event_logs)?;
    let mut proof_logs = validate_logs(proof_logs)?;

    event_logs.sort_by_key(|l| (l.block_number, l.log_index));
    proof_logs.sort_by_key(|l| (l.block_number, l.log_index));

    state
        .populate_timestamps(shutdown, provider, &event_logs, &proof_logs)
        .await
        .context("timestamp enrichment")?;
    if shutdown.is_cancelled() {
        return Ok(());
    }

    let mut event_rows = Vec::with_capacity(event_logs.len());
    for log in event_logs {
        let row = match state.stream {
            Stream::Hub => decode_hub_event_appended(state, log)?,
            Stream::Controller => decode_controller_event_appended(state, log)?,
        };
        event_rows.push(row);
    }

    let mut proof_rows = Vec::with_capacity(proof_logs.len());
    for log in proof_logs {
        let row = decode_tip_proof(state, log)?;
        proof_rows.push(row);
    }

    if await_or_cancel(shutdown, db::insert_batch(dbh, &event_rows, &proof_rows))
        .await?
        .is_none()
    {
        return Ok(());
    }
    Ok(())
}

impl PollState {
    async fn populate_timestamps(
        &mut self,
        shutdown: &CancellationToken,
        provider: &alloy::providers::DynProvider,
        event_logs: &[ValidatedLog],
        proof_logs: &[ValidatedLog],
    ) -> Result<()> {
        let mut blocks: HashSet<u64> = HashSet::new();
        for l in event_logs.iter().chain(proof_logs.iter()) {
            blocks.insert(l.block_number);
            if let Some(ts) = l.block_timestamp {
                self.timestamp_cache
                    .insert(l.block_number, normalize_timestamp_seconds(ts));
            }
        }

        let missing = blocks
            .into_iter()
            .filter(|b| self.timestamp_cache.peek(*b).is_none())
            .collect::<Vec<_>>();

        if missing.is_empty() {
            return Ok(());
        }

        let provider = provider.clone();
        let sem = self.header_sem.clone();
        let shutdown_child = shutdown.clone();
        let concurrency = self.header_concurrency;

        let mut tasks = stream::iter(missing).map(move |block_number| {
            let provider = provider.clone();
            let sem = sem.clone();
            let shutdown = shutdown_child.clone();
            async move {
                tokio::select! {
                    _ = shutdown.cancelled() => Ok::<Option<(u64, u64)>, anyhow::Error>(None),
                    permit = sem.acquire_owned() => {
                        let _permit = permit.expect("semaphore closed");
                        let block = provider
                            .get_block_by_number(alloy::rpc::types::BlockNumberOrTag::Number(block_number))
                            .await
                            .with_context(|| format!("get_block_by_number({block_number})"))?;
                        let Some(block) = block else {
                            anyhow::bail!("block {block_number} not found");
                        };
                        Ok(Some((block_number, normalize_timestamp_seconds(block.header.inner.timestamp))))
                    }
                }
            }
        })
        .buffer_unordered(concurrency);

        while let Some(res) = tasks.next().await {
            if shutdown.is_cancelled() {
                return Ok(());
            }
            if let Some((block_number, ts)) = res? {
                self.timestamp_cache.insert(block_number, ts);
            }
        }

        Ok(())
    }
}

fn decode_hub_event_appended(state: &mut PollState, log: ValidatedLog) -> Result<EventAppendedRow> {
    let block_number = log.block_number;
    let block_timestamp = log
        .block_timestamp
        .map(normalize_timestamp_seconds)
        .or_else(|| state.timestamp_cache.get(block_number))
        .with_context(|| format!("missing block_timestamp for block {block_number}"))?;

    let decoded = log
        .log
        .log_decode::<UntronV3Index::EventAppended>()
        .map_err(|e| anyhow::anyhow!("EventAppended decode failed: {e}"))?;

    let ev = decoded.inner.data;
    let event_seq = u256_to_u64(ev.eventSeq)?;
    let semantic_sig: alloy::primitives::B256 = ev.eventSignature.into();
    let prev_tip: alloy::primitives::B256 = ev.prevTip.into();
    let new_tip: alloy::primitives::B256 = ev.newTip.into();

    let semantic =
        decode::decode_semantic_event(Stream::Hub, semantic_sig, &ev.abiEncodedEventData)?;
    let (event_type, args_json) = semantic.into_db_parts();

    Ok(EventAppendedRow {
        stream: Stream::Hub,
        chain_id: state.chain_id,
        contract_address: state.contract_address_db.clone(),
        block_number: i64::try_from(block_number)
            .context("block_number out of range for bigint")?,
        block_timestamp: i64::try_from(block_timestamp)
            .context("block_timestamp out of range for bigint")?,
        block_hash: log.block_hash,
        tx_hash: log.tx_hash,
        log_index: i32::try_from(log.log_index).context("log_index out of range for int4")?,
        event_seq: i64::try_from(event_seq).context("event_seq out of range for bigint")?,
        prev_tip: domain::Tip(prev_tip),
        new_tip: domain::Tip(new_tip),
        event_signature: domain::EventSignature(semantic_sig),
        abi_encoded_event_data: domain::AbiEncodedEventData(ev.abiEncodedEventData),
        event_type: event_type.into_owned(),
        args_json,
    })
}

fn decode_controller_event_appended(
    state: &mut PollState,
    log: ValidatedLog,
) -> Result<EventAppendedRow> {
    let block_number = log.block_number;
    let block_timestamp = log
        .block_timestamp
        .map(normalize_timestamp_seconds)
        .or_else(|| state.timestamp_cache.get(block_number))
        .with_context(|| format!("missing block_timestamp for block {block_number}"))?;

    let decoded = log
        .log
        .log_decode::<UntronControllerIndex::EventAppended>()
        .map_err(|e| anyhow::anyhow!("EventAppended decode failed: {e}"))?;

    let ev = decoded.inner.data;
    let event_seq = u256_to_u64(ev.eventSeq)?;
    let semantic_sig: alloy::primitives::B256 = ev.eventSignature.into();
    let prev_tip: alloy::primitives::B256 = ev.prevTip.into();
    let new_tip: alloy::primitives::B256 = ev.newTip.into();

    let semantic =
        decode::decode_semantic_event(Stream::Controller, semantic_sig, &ev.abiEncodedEventData)?;
    let (event_type, args_json) = semantic.into_db_parts();

    Ok(EventAppendedRow {
        stream: Stream::Controller,
        chain_id: state.chain_id,
        contract_address: state.contract_address_db.clone(),

        block_number: i64::try_from(block_number)
            .context("block_number out of range for bigint")?,
        block_timestamp: i64::try_from(block_timestamp)
            .context("block_timestamp out of range for bigint")?,
        block_hash: log.block_hash,

        tx_hash: log.tx_hash,
        log_index: i32::try_from(log.log_index).context("log_index out of range for int4")?,

        event_seq: i64::try_from(event_seq).context("event_seq out of range for bigint")?,
        prev_tip: domain::Tip(prev_tip),
        new_tip: domain::Tip(new_tip),
        event_signature: domain::EventSignature(semantic_sig),
        abi_encoded_event_data: domain::AbiEncodedEventData(ev.abiEncodedEventData),

        event_type: event_type.into_owned(),
        args_json,
    })
}

fn decode_tip_proof(state: &mut PollState, log: ValidatedLog) -> Result<ControllerTipProofRow> {
    let domain::ContractAddressDb::Controller(contract_address_db) = &state.contract_address_db
    else {
        anyhow::bail!("internal error: tip proof decoded for non-controller stream");
    };

    let block_number = log.block_number;
    let block_timestamp = log
        .block_timestamp
        .map(normalize_timestamp_seconds)
        .or_else(|| state.timestamp_cache.get(block_number))
        .with_context(|| format!("missing block_timestamp for block {block_number}"))?;

    let decoded = log
        .log
        .log_decode::<UntronControllerIndex::IsEventChainTipCalled>()
        .map_err(|e| anyhow::anyhow!("IsEventChainTipCalled decode failed: {e}"))?;

    Ok(ControllerTipProofRow {
        chain_id: state.chain_id,
        contract_address: contract_address_db.clone(),
        block_number: i64::try_from(block_number)
            .context("block_number out of range for bigint")?,
        block_timestamp: i64::try_from(block_timestamp)
            .context("block_timestamp out of range for bigint")?,
        block_hash: log.block_hash,
        tx_hash: log.tx_hash,
        log_index: i32::try_from(log.log_index).context("log_index out of range for int4")?,
        caller: domain::Caller(decoded.inner.data.caller),
        proved_tip: domain::Tip(decoded.inner.data.eventChainTip.into()),
    })
}

#[derive(Clone)]
struct ValidatedLog {
    log: alloy::rpc::types::Log,
    block_number: u64,
    block_hash: domain::BlockHash,
    tx_hash: domain::TxHash,
    log_index: u32,
    block_timestamp: Option<u64>,
}

fn validate_logs(logs: Vec<alloy::rpc::types::Log>) -> Result<Vec<ValidatedLog>> {
    logs.into_iter()
        .map(|l| {
            let block_timestamp = l.block_timestamp;
            let block_number = l
                .block_number
                .with_context(|| format!("log missing block_number: {:?}", l))?;
            let block_hash = l
                .block_hash
                .with_context(|| format!("log missing block_hash: {:?}", l))?;
            let tx_hash = l
                .transaction_hash
                .with_context(|| format!("log missing transaction_hash: {:?}", l))?;
            let log_index = l
                .log_index
                .with_context(|| format!("log missing log_index: {:?}", l))?;
            let log_index = u32::try_from(log_index).context("log_index out of range for u32")?;
            Ok(ValidatedLog {
                log: l,
                block_number,
                block_hash: domain::BlockHash(block_hash),
                tx_hash: domain::TxHash(tx_hash),
                log_index,
                block_timestamp,
            })
        })
        .collect()
}

fn looks_like_range_too_large(err: &anyhow::Error) -> bool {
    let msg = err.to_string().to_lowercase();
    msg.contains("range too large")
        || msg.contains("block range")
        || msg.contains("too many results")
        || msg.contains("response size exceeded")
        || msg.contains("payload too large")
}

fn looks_like_transient(err: &anyhow::Error) -> bool {
    let msg = err.to_string().to_lowercase();
    msg.contains("timeout")
        || msg.contains("timed out")
        || msg.contains("deadline")
        || (msg.contains("block") && msg.contains("not found"))
        || msg.contains("too many requests")
        || msg.contains("rate limit")
        || msg.contains("429")
        || msg.contains("bad gateway")
        || msg.contains("gateway")
        || msg.contains("service unavailable")
        || msg.contains("503")
        || msg.contains("502")
        || msg.contains("504")
        || msg.contains("connection reset")
        || msg.contains("connection refused")
        || msg.contains("broken pipe")
        || msg.contains("temporarily unavailable")
}

async fn await_or_cancel<T>(
    shutdown: &CancellationToken,
    fut: impl Future<Output = Result<T>>,
) -> Result<Option<T>> {
    tokio::select! {
        _ = shutdown.cancelled() => Ok(None),
        res = fut => Ok(Some(res?)),
    }
}

fn resolve_rpc_contract_address(stream: Stream, contract_address_db: &str) -> Result<Address> {
    match stream {
        Stream::Hub => contract_address_db
            .parse::<Address>()
            .with_context(|| format!("invalid hub contract address: {contract_address_db}")),
        Stream::Controller => util::tron_base58_to_evm_address(contract_address_db),
    }
}

fn u256_to_u64(value: U256) -> Result<u64> {
    u64::try_from(value).with_context(|| format!("U256 too large for u64: {value}"))
}

fn normalize_timestamp_seconds(timestamp: u64) -> u64 {
    // Guardrail for chains/endpoints that return milliseconds since epoch.
    if timestamp >= 20_000_000_000 {
        timestamp / 1000
    } else {
        timestamp
    }
}
