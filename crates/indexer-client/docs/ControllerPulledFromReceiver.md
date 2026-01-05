# ControllerPulledFromReceiver

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_seq** | Option<**i64**> | Controller event sequence for this receiver pull  Note: This is a Primary Key.<pk/> | [optional]
**receiver_salt** | Option<**String**> | Receiver salt identifying which deterministic receiver was swept | [optional]
**token** | Option<**String**> | Token pulled from receiver (Tron address) | [optional]
**token_amount** | Option<[**serde_json::Number**](serde_json::Number.md)> | Raw token amount pulled (uint256) | [optional]
**exchange_rate** | Option<[**serde_json::Number**](serde_json::Number.md)> | Exchange rate used (1e18-scaled); 1:1 for USDT sweeps | [optional]
**usdt_amount** | Option<[**serde_json::Number**](serde_json::Number.md)> | USDT-equivalent amount accounted for this pull (uint256) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


