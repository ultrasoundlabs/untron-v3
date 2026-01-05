# HubTokensRescued

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_seq** | Option<**i64**> | Hub event sequence for this rescue  Note: This is a Primary Key.<pk/> | [optional]
**token** | Option<**String**> | Token rescued (EVM address on hub chain; must not be USDT) | [optional]
**amount** | Option<[**serde_json::Number**](serde_json::Number.md)> | Amount rescued (uint256) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


