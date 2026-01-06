use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
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
    /// UserOperation hash.
    #[schema(example = "0x0000000000000000000000000000000000000000000000000000000000000000")]
    pub userop_hash: String,
    /// Unix timestamp after which the lease is nukeable, computed as `now + duration_seconds`.
    #[schema(example = 1700000000)]
    pub nukeable_after: u64,
}

#[derive(Debug, Deserialize, ToSchema)]
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
