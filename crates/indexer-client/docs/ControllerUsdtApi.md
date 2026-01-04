# \ControllerUsdtApi

All URIs are relative to *http://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**controller_usdt_get**](ControllerUsdtApi.md#controller_usdt_get) | **GET** /controller_usdt | Current controller canonical USDT token (singleton)



## controller_usdt_get

> Vec<models::ControllerUsdt> controller_usdt_get(valid_from_seq, valid_to_seq, usdt, select, order, range, range_unit, offset, limit, prefer)
Current controller canonical USDT token (singleton)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**valid_from_seq** | Option<**String**> | Controller event sequence at which this canonical USDT became current |  |
**valid_to_seq** | Option<**String**> |  |  |
**usdt** | Option<**String**> | Controller canonical USDT token contract (Tron address) |  |
**select** | Option<**String**> | Filtering Columns |  |
**order** | Option<**String**> | Ordering |  |
**range** | Option<**String**> | Limiting and Pagination |  |
**range_unit** | Option<**String**> | Limiting and Pagination |  |[default to items]
**offset** | Option<**String**> | Limiting and Pagination |  |
**limit** | Option<**String**> | Limiting and Pagination |  |
**prefer** | Option<**String**> | Preference |  |

### Return type

[**Vec<models::ControllerUsdt>**](controller_usdt.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.pgrst.object+json;nulls=stripped, application/vnd.pgrst.object+json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

