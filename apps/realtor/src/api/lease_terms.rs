use crate::AppState;
use crate::api::ApiError;
use alloy::primitives::Address;
use axum::http::HeaderMap;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub(super) struct LeaseDefaults {
    pub(super) lease_fee_ppm: u32,
    pub(super) flat_fee: u64,
    pub(super) duration_seconds: u64,
}

#[derive(Debug, Clone)]
pub(super) struct ResolvedLeaseTerms {
    pub(super) defaults: LeaseDefaults,
    pub(super) pair_additional_flat_fees: HashMap<(u64, Address), u64>,
    pub(super) arbitrary_lessee_flat_fee: u64,
}

#[derive(Debug, Deserialize)]
struct LeaseTermsHeader {
    #[serde(default)]
    lease_fee_ppm: Option<u32>,
    #[serde(default)]
    flat_fee: Option<u64>,
    #[serde(default)]
    duration_seconds: Option<u64>,
    #[serde(default)]
    arbitrary_lessee_flat_fee: Option<u64>,
    #[serde(default)]
    pair_additional_flat_fees: Option<Vec<PairAdditionalFlatFeeHeader>>,
}

#[derive(Debug, Deserialize)]
struct PairAdditionalFlatFeeHeader {
    target_chain_id: u64,
    target_token: String,
    additional_flat_fee: u64,
}

pub(super) fn resolve_lease_terms(
    state: &AppState,
    headers: &HeaderMap,
) -> Result<ResolvedLeaseTerms, ApiError> {
    let mut out = ResolvedLeaseTerms {
        defaults: LeaseDefaults {
            lease_fee_ppm: state.cfg.leasing.lease_fee_ppm,
            flat_fee: state.cfg.leasing.flat_fee,
            duration_seconds: state.cfg.leasing.duration_seconds.max(1),
        },
        pair_additional_flat_fees: state.cfg.leasing.pair_additional_flat_fees.clone(),
        arbitrary_lessee_flat_fee: state.cfg.leasing.arbitrary_lessee_flat_fee,
    };

    if !state.cfg.api.lease_terms_header.enabled {
        return Ok(out);
    }

    let header_name = &state.cfg.api.lease_terms_header.header_name;
    let Some(value) = headers.get(header_name) else {
        return Ok(out);
    };

    let raw = value.to_str().map_err(|_| {
        ApiError::BadRequest(format!("{}: must be valid UTF-8", header_name.as_str()))
    })?;
    let raw = raw.trim();
    if raw.is_empty() {
        return Err(ApiError::BadRequest(format!(
            "{}: must be non-empty JSON",
            header_name.as_str()
        )));
    }

    let parsed: LeaseTermsHeader = serde_json::from_str(raw).map_err(|e| {
        ApiError::BadRequest(format!(
            "{}: invalid JSON lease terms object: {e}",
            header_name.as_str()
        ))
    })?;

    if let Some(v) = parsed.lease_fee_ppm {
        out.defaults.lease_fee_ppm = v;
    }
    if let Some(v) = parsed.flat_fee {
        out.defaults.flat_fee = v;
    }
    if let Some(v) = parsed.duration_seconds {
        if v == 0 {
            return Err(ApiError::BadRequest(format!(
                "{}: duration_seconds must be >= 1",
                header_name.as_str()
            )));
        }
        out.defaults.duration_seconds = v;
    }
    if let Some(v) = parsed.arbitrary_lessee_flat_fee {
        out.arbitrary_lessee_flat_fee = v;
    }
    if let Some(v) = parsed.pair_additional_flat_fees {
        out.pair_additional_flat_fees = parse_pair_additional_flat_fees(v)
            .map_err(|msg| ApiError::BadRequest(format!("{}: {msg}", header_name.as_str())))?;
    }

    Ok(out)
}

fn parse_pair_additional_flat_fees(
    v: Vec<PairAdditionalFlatFeeHeader>,
) -> Result<HashMap<(u64, Address), u64>, String> {
    let mut out = HashMap::new();
    for e in v {
        if e.target_chain_id == 0 {
            return Err("pair_additional_flat_fees target_chain_id must be non-zero".to_string());
        }
        let token: Address = e.target_token.parse().map_err(|_| {
            format!(
                "pair_additional_flat_fees target_token invalid address: {}",
                e.target_token
            )
        })?;
        out.insert((e.target_chain_id, token), e.additional_flat_fee);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pair_additional_flat_fees_rejects_zero_chain() {
        let err = parse_pair_additional_flat_fees(vec![PairAdditionalFlatFeeHeader {
            target_chain_id: 0,
            target_token: "0x0000000000000000000000000000000000000001".to_string(),
            additional_flat_fee: 1,
        }])
        .unwrap_err();
        assert!(err.contains("target_chain_id"));
    }

    #[test]
    fn parse_pair_additional_flat_fees_accepts_valid() {
        let map = parse_pair_additional_flat_fees(vec![PairAdditionalFlatFeeHeader {
            target_chain_id: 1,
            target_token: "0x0000000000000000000000000000000000000001".to_string(),
            additional_flat_fee: 7,
        }])
        .unwrap();
        assert_eq!(map.len(), 1);
    }
}
