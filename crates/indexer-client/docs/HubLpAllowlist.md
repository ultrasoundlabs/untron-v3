# HubLpAllowlist

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lp** | Option<**String**> | LP address (EVM)  Note: This is a Primary Key.<pk/> | [optional]
**valid_from_seq** | Option<**i64**> | Event sequence at which this allowlist entry became current  Note: This is a Primary Key.<pk/> | [optional]
**valid_to_seq** | Option<**i64**> |  | [optional]
**allowed** | Option<**bool**> | Whether this LP may deposit into the fast-fill vault (withdrawals are always allowed) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


