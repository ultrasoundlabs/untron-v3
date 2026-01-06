use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Realtor API",
        version = "0.1.0",
        description = "Axum service that creates leases in Untron V3 protocol."
    ),
    paths(
        crate::api::realtor::get_realtor,
        crate::api::realtor::post_realtor,
        crate::api::payout_config::post_payout_config
    ),
    components(
        schemas(
            crate::api::CreateLeaseRequest,
            crate::api::CreateLeaseResponse,
            crate::api::SetPayoutConfigRequest,
            crate::api::SetPayoutConfigResponse,
            crate::api::RealtorInfoResponse,
            crate::api::RealtorTargetPairResponse,
            crate::api::ErrorResponse
        )
    ),
    tags((name = "realtor", description = "Realtor API"))
)]
pub struct RealtorApiDoc;
