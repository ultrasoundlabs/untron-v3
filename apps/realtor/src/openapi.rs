use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Realtor API",
        version = "0.1.0",
        description = "Axum service that creates leases on the hub chain via Safe-4337 userops."
    ),
    paths(crate::api::get_realtor, crate::api::post_realtor),
    components(
        schemas(
            crate::api::CreateLeaseRequest,
            crate::api::CreateLeaseResponse,
            crate::api::RealtorInfoResponse,
            crate::api::ErrorResponse
        )
    ),
    tags((name = "realtor", description = "Realtor API (Axum service)"))
)]
pub struct RealtorApiDoc;
