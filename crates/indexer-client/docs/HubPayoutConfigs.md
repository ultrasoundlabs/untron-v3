# HubPayoutConfigs

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lease_id** | Option<**f64**> | Lease id this payout config applies to  Note: This is a Primary Key.<pk/> | [optional]
**valid_from_seq** | Option<**i32**> | Event sequence at which this payout config became current  Note: This is a Primary Key.<pk/> | [optional]
**valid_to_seq** | Option<**i32**> |  | [optional]
**target_chain_id** | Option<**i32**> | Destination chainId for payouts created under this config | [optional]
**target_token** | Option<**String**> | Settlement token on the hub chain used for fills (USDT or swapped token) | [optional]
**beneficiary** | Option<**String**> | Recipient (EVM) for payouts / bridged delivery | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


