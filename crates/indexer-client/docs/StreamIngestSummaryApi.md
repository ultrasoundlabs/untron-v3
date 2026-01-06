# \StreamIngestSummaryApi

All URIs are relative to *http://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**stream_ingest_summary_get**](StreamIngestSummaryApi.md#stream_ingest_summary_get) | **GET** /stream_ingest_summary | Per-stream ingestion/projection summary for relayers.



## stream_ingest_summary_get

> Vec<models::StreamIngestSummary> stream_ingest_summary_get(stream, applied_through_seq, tip, updated_at, max_event_seq, max_block_number, max_block_timestamp, max_block_time, is_projection_caught_up, select, order, range, range_unit, offset, limit, prefer)
Per-stream ingestion/projection summary for relayers.

This view is intentionally minimal: - It does NOT attempt to query RPC head (that stays in the relayer). - It DOES let relayers detect when projections are behind ingestion (`is_projection_caught_up = false`),   which would make derived \"current state\" views stale.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stream** | Option<**String**> |  |  |
**applied_through_seq** | Option<**String**> |  |  |
**tip** | Option<**String**> |  |  |
**updated_at** | Option<**String**> |  |  |
**max_event_seq** | Option<**String**> |  |  |
**max_block_number** | Option<**String**> |  |  |
**max_block_timestamp** | Option<**String**> |  |  |
**max_block_time** | Option<**String**> |  |  |
**is_projection_caught_up** | Option<**String**> |  |  |
**select** | Option<**String**> | Filtering Columns |  |
**order** | Option<**String**> | Ordering |  |
**range** | Option<**String**> | Limiting and Pagination |  |
**range_unit** | Option<**String**> | Limiting and Pagination |  |[default to items]
**offset** | Option<**String**> | Limiting and Pagination |  |
**limit** | Option<**String**> | Limiting and Pagination |  |
**prefer** | Option<**String**> | Preference |  |

### Return type

[**Vec<models::StreamIngestSummary>**](stream_ingest_summary.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.pgrst.object+json;nulls=stripped, application/vnd.pgrst.object+json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

