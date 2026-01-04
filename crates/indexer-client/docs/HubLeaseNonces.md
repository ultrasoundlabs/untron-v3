# HubLeaseNonces

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lease_id** | Option<**f64**> | Lease id  Note: This is a Primary Key.<pk/> | [optional]
**valid_from_seq** | Option<**i32**> | Event sequence at which this nonce became current  Note: This is a Primary Key.<pk/> | [optional]
**valid_to_seq** | Option<**i32**> |  | [optional]
**nonce** | Option<**f64**> | Current nonce value used for EIP-712 signature replay protection | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


