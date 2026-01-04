# HubLpBalances

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lp** | Option<**String**> | LP address (EVM)  Note: This is a Primary Key.<pk/> | [optional]
**valid_from_seq** | Option<**i32**> | Event sequence at which this balance snapshot became current  Note: This is a Primary Key.<pk/> | [optional]
**valid_to_seq** | Option<**i32**> |  | [optional]
**balance** | Option<**f64**> | Derived LP principal balance (uint256), based on deposits/withdrawals | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


