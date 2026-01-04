# \UnaccountedReceiverUsdtTransfersApi

All URIs are relative to *http://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**unaccounted_receiver_usdt_transfers_get**](UnaccountedReceiverUsdtTransfersApi.md#unaccounted_receiver_usdt_transfers_get) | **GET** /unaccounted_receiver_usdt_transfers | Unaccounted receiver USDT deposits



## unaccounted_receiver_usdt_transfers_get

> Vec<models::UnaccountedReceiverUsdtTransfers> unaccounted_receiver_usdt_transfers_get(chain_id, token, receiver_salt, sender, recipient, amount, block_number, block_timestamp, block_time, block_hash, tx_hash, log_index, expected_lease_id, select, order, range, range_unit, offset, limit, prefer)
Unaccounted receiver USDT deposits

Tron TRC-20 Transfer logs not yet reflected as hub claims  Rows in this view correspond to canonical TRC-20 USDT transfers into deterministic receivers that do NOT yet have a hub-side claim with `origin in (pre-entitle, subjective pre-entitle)` matching `origin_id = tx_hash`.  This view is intended for operators to identify deposits that may require action (preEntitle or receiver pull).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain_id** | Option<**String**> |  |  |
**token** | Option<**String**> |  |  |
**receiver_salt** | Option<**String**> |  |  |
**sender** | Option<**String**> |  |  |
**recipient** | Option<**String**> |  |  |
**amount** | Option<**String**> |  |  |
**block_number** | Option<**String**> |  |  |
**block_timestamp** | Option<**String**> |  |  |
**block_time** | Option<**String**> |  |  |
**block_hash** | Option<**String**> |  |  |
**tx_hash** | Option<**String**> |  |  |
**log_index** | Option<**String**> |  |  |
**expected_lease_id** | Option<**String**> |  |  |
**select** | Option<**String**> | Filtering Columns |  |
**order** | Option<**String**> | Ordering |  |
**range** | Option<**String**> | Limiting and Pagination |  |
**range_unit** | Option<**String**> | Limiting and Pagination |  |[default to items]
**offset** | Option<**String**> | Limiting and Pagination |  |
**limit** | Option<**String**> | Limiting and Pagination |  |
**prefer** | Option<**String**> | Preference |  |

### Return type

[**Vec<models::UnaccountedReceiverUsdtTransfers>**](unaccounted_receiver_usdt_transfers.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.pgrst.object+json;nulls=stripped, application/vnd.pgrst.object+json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

