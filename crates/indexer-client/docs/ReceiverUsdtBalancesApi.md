# \ReceiverUsdtBalancesApi

All URIs are relative to *http://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**receiver_usdt_balances_get**](ReceiverUsdtBalancesApi.md#receiver_usdt_balances_get) | **GET** /receiver_usdt_balances | Derived receiver USDT balances (relayer helper)



## receiver_usdt_balances_get

> Vec<models::ReceiverUsdtBalances> receiver_usdt_balances_get(select, order, range, range_unit, offset, limit, prefer)
Derived receiver USDT balances (relayer helper)

Net receiver USDT balances derived from indexed transfers and pull ledgers. See DB migration `0006_relayer_helpers.sql` for semantics.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**select** | Option<**String**> | Filtering Columns |  |
**order** | Option<**String**> | Ordering |  |
**range** | Option<**String**> | Limiting and Pagination |  |
**range_unit** | Option<**String**> | Limiting and Pagination |  |[default to items]
**offset** | Option<**String**> | Limiting and Pagination |  |
**limit** | Option<**String**> | Limiting and Pagination |  |
**prefer** | Option<**String**> | Preference |  |

### Return type

[**Vec<models::ReceiverUsdtBalances>**](receiver_usdt_balances.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.pgrst.object+json;nulls=stripped, application/vnd.pgrst.object+json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

