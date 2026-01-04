# \StreamCursorApi

All URIs are relative to *http://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**stream_cursor_get**](StreamCursorApi.md#stream_cursor_get) | **GET** /stream_cursor | Projection cursors (per stream)



## stream_cursor_get

> Vec<models::StreamCursor> stream_cursor_get(stream, applied_through_seq, tip, updated_at, select, order, range, range_unit, offset, limit, prefer)
Projection cursors (per stream)

Shows how far the database has applied canonical events to derived state tables.  Fields: - `applied_through_seq`: highest contiguous canonical event_seq applied - `tip`: expected prev_tip for the next event - `updated_at`: last time the cursor advanced/rolled back

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stream** | Option<**String**> | Stream name (`hub` or `controller`) |  |
**applied_through_seq** | Option<**String**> | Highest contiguous canonical event sequence already applied to derived tables |  |
**tip** | Option<**String**> | Expected `prev_tip` for the next event to apply (hash-chain continuity check) |  |
**updated_at** | Option<**String**> | Timestamp when the cursor last advanced or rolled back |  |
**select** | Option<**String**> | Filtering Columns |  |
**order** | Option<**String**> | Ordering |  |
**range** | Option<**String**> | Limiting and Pagination |  |
**range_unit** | Option<**String**> | Limiting and Pagination |  |[default to items]
**offset** | Option<**String**> | Limiting and Pagination |  |
**limit** | Option<**String**> | Limiting and Pagination |  |
**prefer** | Option<**String**> | Preference |  |

### Return type

[**Vec<models::StreamCursor>**](stream_cursor.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.pgrst.object+json;nulls=stripped, application/vnd.pgrst.object+json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

