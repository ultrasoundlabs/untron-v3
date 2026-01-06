use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Realtor API",
        version = "0.1.0",
        description = "Axum service that creates leases in Untron V3 protocol."
    ),
    paths(crate::api::get_realtor, crate::api::post_realtor),
    components(
        schemas(
            crate::api::CreateLeaseRequest,
            crate::api::CreateLeaseResponse,
            crate::api::RealtorInfoResponse,
            crate::api::RealtorTargetPairResponse,
            crate::api::ErrorResponse
        )
    ),
    tags((name = "realtor", description = "Realtor API"))
)]
pub struct RealtorApiDoc;
