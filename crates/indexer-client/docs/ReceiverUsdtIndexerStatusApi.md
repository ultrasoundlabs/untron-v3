# \ReceiverUsdtIndexerStatusApi

All URIs are relative to *http://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**receiver_usdt_indexer_status_get**](ReceiverUsdtIndexerStatusApi.md#receiver_usdt_indexer_status_get) | **GET** /receiver_usdt_indexer_status | Receiver USDT transfer indexer status (relayer helper)



## receiver_usdt_indexer_status_get

> Vec<models::ReceiverUsdtIndexerStatus> receiver_usdt_indexer_status_get(select, order, range, range_unit, offset, limit, prefer)
Receiver USDT transfer indexer status (relayer helper)

Receiver USDT transfer indexer status (tail + backfill). See DB migration `0006_relayer_helpers.sql` for semantics.

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

[**Vec<models::ReceiverUsdtIndexerStatus>**](receiver_usdt_indexer_status.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.pgrst.object+json;nulls=stripped, application/vnd.pgrst.object+json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

