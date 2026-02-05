use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateLeaseRequest {
    #[serde(default)]
    /// Optional receiver salt (bytes32 hex).
    ///
    /// - If omitted, server selects an available salt.
    /// - If provided, must exist in indexer `receiver_salt_candidates`.
    #[schema(
        example = "0x0000000000000000000000000000000000000000000000000000000000000000",
        pattern = "^0x[0-9a-fA-F]{64}$",
        nullable = true
    )]
    pub receiver_salt: Option<String>,

    #[serde(default)]
    /// Optional lessee address.
    ///
    /// - If omitted, the zero address is used.
    /// - If provided, `arbitrary_lessee_flat_fee` is added to the flat fee.
    #[schema(
        example = "0x0000000000000000000000000000000000000001",
        pattern = "^0x[0-9a-fA-F]{40}$",
        nullable = true
    )]
    pub lessee: Option<String>,

    /// Required lease duration in seconds.
    ///
    /// Must be `<= max_duration_seconds` when `max_duration_seconds != 0`.
    #[schema(example = 2592000, minimum = 1)]
    pub duration_seconds: u64,

    /// Destination EVM chainId.
    ///
    /// Must have a configured bridger route for `(target_chain_id,target_token)`.
    #[schema(example = 1, minimum = 1)]
    pub target_chain_id: u64,

    /// Target settlement token (EVM address on hub chain).
    ///
    /// Must have a configured bridger route for `(target_chain_id,target_token)`.
    #[schema(
        example = "0x0000000000000000000000000000000000000002",
        pattern = "^0x[0-9a-fA-F]{40}$"
    )]
    pub target_token: String,

    /// Beneficiary address (EVM).
    #[schema(
        example = "0x0000000000000000000000000000000000000003",
        pattern = "^0x[0-9a-fA-F]{40}$"
    )]
    pub beneficiary: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateLeaseResponse {
    /// Receiver salt selected/used for the lease (bytes32 hex).
    #[schema(
        example = "0x0000000000000000000000000000000000000000000000000000000000000000",
        pattern = "^0x[0-9a-fA-F]{64}$"
    )]
    pub receiver_salt: String,

    /// Deterministically-derived receiver address (Tron base58).
    ///
    /// This is derived from `receiver_salt` using the controller's receiver scheme, and may be
    /// absent if the realtor is not configured with the necessary hub + Tron RPC settings.
    #[schema(example = "TX9xZ4mV2h4h9qv7q8qXbW1d7m8m1y1y1y")]
    pub receiver_address_tron: Option<String>,

    /// Deterministically-derived receiver address (EVM checksum address on hub chain).
    ///
    /// This is derived from `receiver_salt` using the controller's receiver scheme, and may be
    /// absent if the realtor is not configured with the necessary hub + Tron RPC settings.
    #[schema(example = "0x0000000000000000000000000000000000000000")]
    pub receiver_address_evm: Option<String>,

    /// UserOperation hash.
    #[schema(example = "0x0000000000000000000000000000000000000000000000000000000000000000")]
    pub userop_hash: String,

    /// Global lease id (uint256) assigned by the hub contract.
    ///
    /// This service waits for a userop receipt (or, failing that, the indexer) before returning.
    #[schema(example = 1, minimum = 1)]
    pub lease_id: u64,

    /// Unix timestamp after which the lease is nukeable, computed as `now + duration_seconds`.
    #[schema(example = 1700000000)]
    pub nukeable_after: u64,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct SetPayoutConfigRequest {
    /// Lease id to update.
    #[schema(example = 1, minimum = 1)]
    pub lease_id: u64,

    /// Destination EVM chainId.
    #[schema(example = 1, minimum = 1)]
    pub target_chain_id: u64,

    /// Target settlement token (EVM address on hub chain).
    #[schema(
        example = "0x0000000000000000000000000000000000000002",
        pattern = "^0x[0-9a-fA-F]{40}$"
    )]
    pub target_token: String,

    /// Beneficiary address (EVM).
    #[schema(
        example = "0x0000000000000000000000000000000000000003",
        pattern = "^0x[0-9a-fA-F]{40}$"
    )]
    pub beneficiary: String,

    /// EIP-712 signature deadline (unix seconds).
    #[schema(example = 1700000000)]
    pub deadline: u64,

    /// EIP-712 signature bytes (0x hex).
    ///
    /// Typically a 65-byte ECDSA signature for EOAs; contract lessees may use ERC-1271 signatures.
    #[schema(example = "0x")]
    pub signature: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SetPayoutConfigResponse {
    /// UserOperation hash.
    #[schema(example = "0x0000000000000000000000000000000000000000000000000000000000000000")]
    pub userop_hash: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RealtorTargetPairResponse {
    /// Destination EVM chainId.
    #[schema(example = 1, minimum = 1)]
    pub target_chain_id: u64,
    /// Target settlement token (EVM address on hub chain).
    #[schema(
        example = "0x0000000000000000000000000000000000000002",
        pattern = "^0x[0-9a-fA-F]{40}$"
    )]
    pub target_token: String,
    /// Effective lease fee in PPM for this pair (currently not pair-specific).
    #[schema(example = 10000)]
    pub effective_fee_ppm: u32,
    /// Effective flat fee for this pair including any env-configured per-pair additional flat fee.
    ///
    /// Does not include `arbitrary_lessee_flat_fee` (which depends on the request).
    #[schema(example = 0)]
    pub effective_flat_fee: u64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RealtorInfoResponse {
    /// Optional upstream principal/user identifier echoed from `x-untron-principal-id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = true, example = "acct_123")]
    pub user: Option<String>,

    /// Address that identifies this realtor (the configured hub Safe).
    #[schema(
        example = "0x0000000000000000000000000000000000000004",
        pattern = "^0x[0-9a-fA-F]{40}$"
    )]
    pub realtor_address: String,
    /// UntronV3 contract address on hub chain.
    #[schema(
        example = "0x0000000000000000000000000000000000000005",
        pattern = "^0x[0-9a-fA-F]{40}$"
    )]
    pub untron_v3: String,

    /// Whether this realtor is allowlisted on the hub.
    ///
    /// When false, `POST /realtor` returns `403`.
    #[schema(example = true)]
    pub allowed: bool,

    /// Minimum lease fee PPM configured on hub for this realtor.
    #[schema(example = 0)]
    pub min_fee_ppm: u32,
    /// Minimum flat fee configured on hub for this realtor.
    #[schema(example = 0)]
    pub min_flat_fee: u64,
    /// Maximum allowed lease duration in seconds.
    ///
    /// If 0, no max is enforced by this service.
    #[schema(example = 2592000)]
    pub max_duration_seconds: u64,

    /// Rate limit: max leases in window.
    #[schema(example = 0)]
    pub lease_rate_max_leases: u64,
    /// Rate limit: window size in seconds.
    #[schema(example = 0)]
    pub lease_rate_window_seconds: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Rate limit: remaining leases in window (if reported by indexer).
    #[schema(nullable = true)]
    pub lease_rate_remaining: Option<u64>,

    /// Default lease fee PPM from this service's env.
    #[schema(example = 10000)]
    pub default_fee_ppm: u32,
    /// Default flat fee from this service's env (before min and adders).
    #[schema(example = 0)]
    pub default_flat_fee: u64,
    /// Default duration seconds from this service's env (used only to compute effective_duration_seconds).
    #[schema(example = 2592000)]
    pub default_duration_seconds: u64,

    /// Effective duration seconds used for informational purposes (currently derived from defaults and max).
    ///
    /// `POST /realtor` requires `duration_seconds` explicitly.
    #[schema(example = 2592000)]
    pub effective_duration_seconds: u64,

    /// Supported (target_chain_id,target_token) pairs from the current bridger routing table.
    pub supported_pairs: Vec<RealtorTargetPairResponse>,

    /// Additional flat fee added when a non-null `lessee` is provided in `POST /realtor`.
    #[schema(example = 0)]
    pub arbitrary_lessee_flat_fee: u64,
}

/// Realtor-side aggregated view of a lease in the Untron V3 protocol.
///
/// This response is designed to be stable for clients:
/// - Uint256-like values are returned as decimal strings.
/// - Source-of-truth data comes from the indexer API (PostgREST views).
#[derive(Debug, Serialize, ToSchema)]
pub struct LeaseViewResponse {
    /// Lease id (uint256, decimal string).
    #[schema(example = "1")]
    pub lease_id: String,

    /// Receiver salt (bytes32 hex).
    #[schema(
        example = "0x0000000000000000000000000000000000000000000000000000000000000000",
        pattern = "^0x[0-9a-fA-F]{64}$"
    )]
    pub receiver_salt: String,

    /// Receiver address on Tron (base58check, T...).
    ///
    /// Derived from `receiver_salt` using the controller's deterministic receiver scheme.
    #[schema(nullable = true, example = "T9yD14Nj9j7xAB4dbGeiX9h8unkKHxuWwb")]
    pub receiver_address_tron: Option<String>,

    /// Receiver address in EVM hex form (0x...).
    ///
    /// This is the 20-byte address corresponding to the Tron receiver address.
    #[schema(
        nullable = true,
        example = "0x0000000000000000000000000000000000000000",
        pattern = "^0x[0-9a-fA-F]{40}$"
    )]
    pub receiver_address_evm: Option<String>,

    /// Realtor (EVM address) that created this lease.
    #[schema(
        example = "0x0000000000000000000000000000000000000004",
        pattern = "^0x[0-9a-fA-F]{40}$"
    )]
    pub realtor: String,

    /// Whether this lease was created by this realtor service instance.
    #[schema(example = true)]
    pub is_owned_by_this_realtor: bool,

    /// Current lessee (EVM address) that controls payout config updates.
    #[schema(
        example = "0x0000000000000000000000000000000000000001",
        pattern = "^0x[0-9a-fA-F]{40}$"
    )]
    pub lessee: String,

    /// Lease start time on hub chain (unix seconds).
    #[schema(example = 1700000000)]
    pub start_time: u64,

    /// Earliest timestamp when the lease is nukeable (unix seconds).
    #[schema(example = 1700000000)]
    pub nukeable_after: u64,

    /// Lease fee (ppm).
    #[schema(example = 10000)]
    pub lease_fee_ppm: u32,

    /// Flat fee (USDT units) (uint256, decimal string).
    #[schema(example = "0")]
    pub flat_fee: String,

    /// Current per-lease nonce used for payout config signatures (uint256, decimal string).
    #[schema(example = "0")]
    pub lease_nonce: String,

    /// Current payout config (if available in indexer).
    #[schema(nullable = true)]
    pub payout_config_current: Option<LeasePayoutConfigView>,

    /// Payout config history (KV versions ordered by valid_from_seq).
    pub payout_config_history: Vec<LeasePayoutConfigVersionView>,

    /// Claims emitted by this lease (current state per claim_id).
    pub claims: Vec<LeaseClaimView>,

    /// Total number of claims.
    #[schema(example = 0)]
    pub claims_total: u64,

    /// Number of filled claims.
    #[schema(example = 0)]
    pub claims_filled: u64,

    /// Canonical receiver USDT deposits that are still eligible for `preEntitle`
    /// and have not yet been accounted for by a hub pre-entitle claim.
    ///
    /// Entries are a minimal, stable subset:
    /// `{ tx_hash, sender, amount, block_timestamp, log_index }`.
    pub pending_usdt_deposits: Vec<UsdtDepositAttributionEntryView>,

    /// Total number of pending deposits.
    #[schema(example = 0)]
    pub pending_usdt_deposits_total: u64,

    /// Sum of pending deposit amounts (uint256, decimal string).
    #[schema(example = "0")]
    pub pending_usdt_deposits_amount: String,

    /// Latest pending deposit Tron block timestamp (seconds).
    #[schema(example = 0)]
    pub pending_usdt_deposits_latest_block_timestamp: i64,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct LeasePayoutConfigView {
    #[schema(example = 1, minimum = 1)]
    pub target_chain_id: u64,
    #[schema(
        example = "0x0000000000000000000000000000000000000002",
        pattern = "^0x[0-9a-fA-F]{40}$"
    )]
    pub target_token: String,
    #[schema(
        example = "0x0000000000000000000000000000000000000003",
        pattern = "^0x[0-9a-fA-F]{40}$"
    )]
    pub beneficiary: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LeasePayoutConfigVersionView {
    pub config: LeasePayoutConfigView,
    #[schema(example = 0)]
    pub valid_from_seq: u64,
    #[schema(nullable = true)]
    pub valid_to_seq: Option<u64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UsdtDepositAttributionEntryView {
    /// Tron txId (sha256(raw_data)).
    #[schema(example = "0x0000000000000000000000000000000000000000000000000000000000000000")]
    pub tx_hash: String,
    /// TRC-20 `Transfer.from` (Tron base58check, T...).
    #[schema(example = "T9yD14Nj9j7xAB4dbGeiX9h8unkKHxuWwb")]
    pub sender: String,
    /// Attributed raw USDT amount (uint256, decimal string).
    #[schema(example = "0")]
    pub amount: String,
    /// Tron block timestamp (seconds) for the underlying transfer log.
    #[schema(example = 0)]
    pub block_timestamp: i64,
    /// TRC-20 Transfer log index within the tx.
    #[schema(example = 0)]
    pub log_index: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LeaseClaimView {
    /// Claim id (uint256, decimal string).
    #[schema(example = "0")]
    pub claim_id: String,
    /// Claim lifecycle status.
    #[schema(example = "created")]
    pub status: String,
    /// Queue index (uint256, decimal string).
    #[schema(example = "0")]
    pub queue_index: String,
    /// USDT-denominated claim amount (uint256, decimal string).
    #[schema(example = "0")]
    pub amount_usdt: String,
    #[schema(example = 1, minimum = 1)]
    pub target_chain_id: u64,
    #[schema(
        example = "0x0000000000000000000000000000000000000002",
        pattern = "^0x[0-9a-fA-F]{40}$"
    )]
    pub target_token: String,
    #[schema(
        example = "0x0000000000000000000000000000000000000003",
        pattern = "^0x[0-9a-fA-F]{40}$"
    )]
    pub beneficiary: String,
    /// Origin code (matches `UntronV3Index.ClaimOrigin`).
    #[schema(example = 0)]
    pub origin: i32,
    /// Origin identifier (txId for pre-entitle, receiver_salt for receiver pull, etc.)
    #[schema(example = "0x")]
    pub origin_id: String,
    /// Origin actor (EVM address).
    #[schema(example = "0x0000000000000000000000000000000000000000")]
    pub origin_actor: String,
    /// Origin token/address (string; Tron token for receiver pull; zero address otherwise).
    #[schema(example = "0x0000000000000000000000000000000000000000")]
    pub origin_token: String,
    /// Origin timestamp (seconds).
    #[schema(example = 0)]
    pub origin_timestamp: i64,
    /// Raw amount before fees (uint256, decimal string).
    #[schema(example = "0")]
    pub origin_raw_amount: String,
    /// Best-effort attribution of this claim's underlying USDT deposits.
    ///
    /// - For pre-entitle origins, this is usually a single entry.
    /// - For receiver-pull origins, this may contain multiple entries (FIFO approximation) or be empty.
    pub usdt_deposit_attribution: Vec<UsdtDepositAttributionEntryView>,

    /// Hub transaction hash that filled this claim (when `status == "filled"`).
    ///
    /// Derived from the canonical hub `ClaimFilled` event.
    #[schema(
        nullable = true,
        example = "0x0000000000000000000000000000000000000000000000000000000000000000"
    )]
    pub fill_tx_hash: Option<String>,
    #[schema(example = 0)]
    pub valid_from_seq: u64,
    #[schema(nullable = true)]
    pub valid_to_seq: Option<u64>,
}
