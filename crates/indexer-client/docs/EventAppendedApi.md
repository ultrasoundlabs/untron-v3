# \EventAppendedApi

All URIs are relative to *http://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**event_appended_get**](EventAppendedApi.md#event_appended_get) | **GET** /event_appended | Canonical raw EventAppended stream (both hub + controller)



## event_appended_get

> Vec<models::EventAppended> event_appended_get(stream, event_seq, prev_tip, new_tip, event_signature, abi_encoded_event_data, event_type, args, block_number, block_timestamp, block_time, block_hash, tx_hash, log_index, select, order, range, range_unit, offset, limit, prefer)
Canonical raw EventAppended stream (both hub + controller)

This is the canonical ordered stream of Untron \"semantic events\", as emitted by the onchain index contracts. Each row corresponds to one onchain `EventAppended` log and includes: - hash-chain linkage (prev_tip/new_tip/event_seq) - the semantic event name (`event_type`) and decoded arguments (`args`) - block/tx metadata for auditability  This view filters to `canonical=true` only.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stream** | Option<**String**> | Which stream emitted this EventAppended log (`hub` or `controller`) |  |
**event_seq** | Option<**String**> | Monotonic sequence number in the stream's onchain event hash-chain |  |
**prev_tip** | Option<**String**> | Previous onchain event hash-chain tip before this event |  |
**new_tip** | Option<**String**> | New onchain event hash-chain tip after this event |  |
**event_signature** | Option<**String**> | Keccak256 hash of the semantic event signature string (bytes32 hex) |  |
**abi_encoded_event_data** | Option<**String**> | Exact ABI-encoded event payload bytes that were hashed onchain (0x hex) |  |
**event_type** | Option<**String**> | Worker-decoded semantic event name (e.g. `LeaseCreated`, `ClaimCreated`) |  |
**args** | Option<**String**> | Worker-decoded event arguments as JSON (snake_case keys; values as strings/hex) |  |
**block_number** | Option<**String**> | Block number containing the EventAppended log |  |
**block_timestamp** | Option<**String**> | Block timestamp (seconds since epoch) containing the EventAppended log |  |
**block_time** | Option<**String**> | Convenience timestamp (block_timestamp converted to timestamptz) |  |
**block_hash** | Option<**String**> | Block hash of the log's block (bytes32 hex) |  |
**tx_hash** | Option<**String**> | Transaction hash of the log's transaction (bytes32 hex) |  |
**log_index** | Option<**String**> | Log index within the transaction receipt (0-based) |  |
**select** | Option<**String**> | Filtering Columns |  |
**order** | Option<**String**> | Ordering |  |
**range** | Option<**String**> | Limiting and Pagination |  |
**range_unit** | Option<**String**> | Limiting and Pagination |  |[default to items]
**offset** | Option<**String**> | Limiting and Pagination |  |
**limit** | Option<**String**> | Limiting and Pagination |  |
**prefer** | Option<**String**> | Preference |  |

### Return type

[**Vec<models::EventAppended>**](event_appended.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.pgrst.object+json;nulls=stripped, application/vnd.pgrst.object+json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

