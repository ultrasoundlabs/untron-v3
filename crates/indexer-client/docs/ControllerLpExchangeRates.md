# ControllerLpExchangeRates

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**token** | Option<**String**> | Token address on Tron whose exchange rate is configured  Note: This is a Primary Key.<pk/> | [optional]
**valid_from_seq** | Option<**i32**> | Controller event sequence at which this exchange rate became current  Note: This is a Primary Key.<pk/> | [optional]
**valid_to_seq** | Option<**i32**> |  | [optional]
**exchange_rate** | Option<**f64**> | Scaled exchange rate used to convert token amounts into USDT-equivalent amounts | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


