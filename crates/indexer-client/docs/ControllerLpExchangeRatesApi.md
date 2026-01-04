# \ControllerLpExchangeRatesApi

All URIs are relative to *http://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**controller_lp_exchange_rates_get**](ControllerLpExchangeRatesApi.md#controller_lp_exchange_rates_get) | **GET** /controller_lp_exchange_rates | Current LP exchange rates for non-USDT tokens (KV)



## controller_lp_exchange_rates_get

> Vec<models::ControllerLpExchangeRates> controller_lp_exchange_rates_get(token, valid_from_seq, valid_to_seq, exchange_rate, select, order, range, range_unit, offset, limit, prefer)
Current LP exchange rates for non-USDT tokens (KV)

Used by controller sweeps to compute USDT-equivalent amounts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Token address on Tron whose exchange rate is configured |  |
**valid_from_seq** | Option<**String**> | Controller event sequence at which this exchange rate became current |  |
**valid_to_seq** | Option<**String**> |  |  |
**exchange_rate** | Option<**String**> | Scaled exchange rate used to convert token amounts into USDT-equivalent amounts |  |
**select** | Option<**String**> | Filtering Columns |  |
**order** | Option<**String**> | Ordering |  |
**range** | Option<**String**> | Limiting and Pagination |  |
**range_unit** | Option<**String**> | Limiting and Pagination |  |[default to items]
**offset** | Option<**String**> | Limiting and Pagination |  |
**limit** | Option<**String**> | Limiting and Pagination |  |
**prefer** | Option<**String**> | Preference |  |

### Return type

[**Vec<models::ControllerLpExchangeRates>**](controller_lp_exchange_rates.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.pgrst.object+json;nulls=stripped, application/vnd.pgrst.object+json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

