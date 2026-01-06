# \ReceiverUsdtIndexerStatusApi

All URIs are relative to *http://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**receiver_usdt_indexer_status_get**](ReceiverUsdtIndexerStatusApi.md#receiver_usdt_indexer_status_get) | **GET** /receiver_usdt_indexer_status | Receiver USDT transfer indexer status (tail + backfill).



## receiver_usdt_indexer_status_get

> Vec<models::ReceiverUsdtIndexerStatus> receiver_usdt_indexer_status_get(stream, tail_next_block, tail_updated_at, min_backfill_next_block, receiver_count, backfill_pending_receivers, watchlist_updated_at, select, order, range, range_unit, offset, limit, prefer)
Receiver USDT transfer indexer status (tail + backfill).

The TRC-20 receiver transfer indexer has two moving parts: - a single shared tail cursor (`ctl.receiver_usdt_tail_cursor.next_block`), and - per-receiver backfill cursors (`ctl.receiver_watchlist.backfill_next_block`).  Relayers should generally avoid acting on receiver-transfer-derived state unless: - `min_backfill_next_block` is NULL (no pending backfills), and - `tail_next_block` is close to the Tron head (with the relayer's own head check).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stream** | Option<**String**> |  |  |
**tail_next_block** | Option<**String**> |  |  |
**tail_updated_at** | Option<**String**> |  |  |
**min_backfill_next_block** | Option<**String**> |  |  |
**receiver_count** | Option<**String**> |  |  |
**backfill_pending_receivers** | Option<**String**> |  |  |
**watchlist_updated_at** | Option<**String**> |  |  |
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

