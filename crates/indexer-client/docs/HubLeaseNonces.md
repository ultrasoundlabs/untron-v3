# HubLeaseNonces

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lease_id** | Option<[**serde_json::Number**](serde_json::Number.md)> | Lease id  Note: This is a Primary Key.<pk/> | [optional]
**valid_from_seq** | Option<**i64**> | Event sequence at which this nonce became current  Note: This is a Primary Key.<pk/> | [optional]
**valid_to_seq** | Option<**i64**> |  | [optional]
**nonce** | Option<[**serde_json::Number**](serde_json::Number.md)> | Current nonce value used for EIP-712 signature replay protection | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


