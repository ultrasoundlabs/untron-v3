# HubSwapRates

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**target_token** | Option<**String**> | Settlement token (EVM) on the hub chain  Note: This is a Primary Key.<pk/> | [optional]
**valid_from_seq** | Option<**i64**> | Event sequence at which this rate became current  Note: This is a Primary Key.<pk/> | [optional]
**valid_to_seq** | Option<**i64**> |  | [optional]
**rate_ppm** | Option<**i64**> | Expected output rate: targetToken units per 1e6 USDT units | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


