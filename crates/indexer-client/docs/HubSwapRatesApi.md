# \HubSwapRatesApi

All URIs are relative to *http://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**hub_swap_rates_get**](HubSwapRatesApi.md#hub_swap_rates_get) | **GET** /hub_swap_rates | Current swap rates for settlement tokens (KV)



## hub_swap_rates_get

> Vec<models::HubSwapRates> hub_swap_rates_get(target_token, valid_from_seq, valid_to_seq, rate_ppm, select, order, range, range_unit, offset, limit, prefer)
Current swap rates for settlement tokens (KV)

Rates are used by fillers to swap USDT into the target token during claim settlement.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target_token** | Option<**String**> | Settlement token (EVM) on the hub chain |  |
**valid_from_seq** | Option<**String**> | Event sequence at which this rate became current |  |
**valid_to_seq** | Option<**String**> |  |  |
**rate_ppm** | Option<**String**> | Expected output rate: targetToken units per 1e6 USDT units |  |
**select** | Option<**String**> | Filtering Columns |  |
**order** | Option<**String**> | Ordering |  |
**range** | Option<**String**> | Limiting and Pagination |  |
**range_unit** | Option<**String**> | Limiting and Pagination |  |[default to items]
**offset** | Option<**String**> | Limiting and Pagination |  |
**limit** | Option<**String**> | Limiting and Pagination |  |
**prefer** | Option<**String**> | Preference |  |

### Return type

[**Vec<models::HubSwapRates>**](hub_swap_rates.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.pgrst.object+json;nulls=stripped, application/vnd.pgrst.object+json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

