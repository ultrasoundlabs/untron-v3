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
        crate::api::payout_config::post_payout_config,
        crate::api::leases::get_lease
    ),
    components(
        schemas(
            crate::api::CreateLeaseRequest,
            crate::api::CreateLeaseResponse,
            crate::api::SetPayoutConfigRequest,
            crate::api::SetPayoutConfigResponse,
            crate::api::RealtorInfoResponse,
            crate::api::RealtorTargetPairResponse,
            crate::api::LeaseViewResponse,
            crate::api::LeasePayoutConfigView,
            crate::api::LeasePayoutConfigVersionView,
            crate::api::LeaseClaimView,
            crate::api::ErrorResponse
        )
    ),
    tags((name = "realtor", description = "Realtor API"))
)]
pub struct RealtorApiDoc;

#[cfg(test)]
mod tests {
    use super::RealtorApiDoc;
    use utoipa::OpenApi;

    #[test]
    fn openapi_includes_pending_usdt_deposits_fields() {
        let v = serde_json::to_value(RealtorApiDoc::openapi()).expect("openapi json");
        let props = &v["components"]["schemas"]["LeaseViewResponse"]["properties"];

        assert!(
            props.get("pending_usdt_deposits").is_some(),
            "missing pending_usdt_deposits"
        );
        assert!(
            props.get("pending_usdt_deposits_total").is_some(),
            "missing pending_usdt_deposits_total"
        );
        assert!(
            props.get("pending_usdt_deposits_amount").is_some(),
            "missing pending_usdt_deposits_amount"
        );
        assert!(
            props
                .get("pending_usdt_deposits_latest_block_timestamp")
                .is_some(),
            "missing pending_usdt_deposits_latest_block_timestamp"
        );
    }

    #[test]
    fn openapi_includes_claim_fill_tx_hash() {
        let v = serde_json::to_value(RealtorApiDoc::openapi()).expect("openapi json");
        let props = &v["components"]["schemas"]["LeaseClaimView"]["properties"];

        assert!(
            props.get("fill_tx_hash").is_some(),
            "missing fill_tx_hash on LeaseClaimView"
        );
    }
}
