# \ControllerTipProofsApi

All URIs are relative to *http://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**controller_tip_proofs_get**](ControllerTipProofsApi.md#controller_tip_proofs_get) | **GET** /controller_tip_proofs | Controller tip proof logs (IsEventChainTipCalled)



## controller_tip_proofs_get

> Vec<models::ControllerTipProofs> controller_tip_proofs_get(block_number, block_timestamp, block_hash, block_time, tx_hash, log_index, caller, proved_tip, select, order, range, range_unit, offset, limit, prefer)
Controller tip proof logs (IsEventChainTipCalled)

On Tron, the controller exposes `isEventChainTip(bytes32)` which emits `IsEventChainTipCalled`. This event is NOT appended into the controller's hash chain; it is used as a proof-carrying log to anchor controller event sequences.  This view filters to `canonical=true` only.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**block_number** | Option<**String**> | Block number containing the controller `IsEventChainTipCalled` log |  |
**block_timestamp** | Option<**String**> | Block timestamp (seconds since epoch) containing the log |  |
**block_hash** | Option<**String**> | Block hash of the log's block (bytes32 hex) |  |
**block_time** | Option<**String**> | Convenience timestamp (block_timestamp converted to timestamptz) |  |
**tx_hash** | Option<**String**> | Transaction hash of the transaction containing the log (bytes32 hex) |  |
**log_index** | Option<**String**> | Log index within the transaction receipt (0-based) |  |
**caller** | Option<**String**> | Tron address that called `isEventChainTip(bytes32)` |  |
**proved_tip** | Option<**String**> | Hash-chain tip value that the caller asserted as the controller's current tip |  |
**select** | Option<**String**> | Filtering Columns |  |
**order** | Option<**String**> | Ordering |  |
**range** | Option<**String**> | Limiting and Pagination |  |
**range_unit** | Option<**String**> | Limiting and Pagination |  |[default to items]
**offset** | Option<**String**> | Limiting and Pagination |  |
**limit** | Option<**String**> | Limiting and Pagination |  |
**prefer** | Option<**String**> | Preference |  |

### Return type

[**Vec<models::ControllerTipProofs>**](controller_tip_proofs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.pgrst.object+json;nulls=stripped, application/vnd.pgrst.object+json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

