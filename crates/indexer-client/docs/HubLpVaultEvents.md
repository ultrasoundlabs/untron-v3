# HubLpVaultEvents

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_seq** | Option<**i32**> | Hub event sequence for this vault event  Note: This is a Primary Key.<pk/> | [optional]
**kind** | Option<**String**> | `deposit` or `withdraw` | [optional]
**lp** | Option<**String**> | LP address (EVM) | [optional]
**amount** | Option<**f64**> | Amount deposited/withdrawn (uint256, USDT units) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


