# ControllerUsdtRebalanced

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_seq** | Option<**i64**> | Controller event sequence for this rebalance  Note: This is a Primary Key.<pk/> | [optional]
**in_amount** | Option<[**serde_json::Number**](serde_json::Number.md)> | USDT amount bridged in (uint256) | [optional]
**out_amount** | Option<[**serde_json::Number**](serde_json::Number.md)> | Expected USDT amount out on destination (uint256) | [optional]
**rebalancer** | Option<**String**> | Rebalancer used (Tron address) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


