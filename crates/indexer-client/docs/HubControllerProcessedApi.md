# \HubControllerProcessedApi

All URIs are relative to *http://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**hub_controller_processed_get**](HubControllerProcessedApi.md#hub_controller_processed_get) | **GET** /hub_controller_processed | Hub-side controller processed events ledger (append-only)



## hub_controller_processed_get

> Vec<models::HubControllerProcessed> hub_controller_processed_get(event_seq, event_index, block_number, block_timestamp, event_signature, abi_encoded_event_data, select, order, range, range_unit, offset, limit, prefer)
Hub-side controller processed events ledger (append-only)

Records that the hub processed a queued controller event during reconciliation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_seq** | Option<**String**> | Hub event sequence for this controller processing record |  |
**event_index** | Option<**String**> | Index within the hub's controller event queue that was processed |  |
**block_number** | Option<**String**> | Controller event block number (as embedded in the hub event payload) |  |
**block_timestamp** | Option<**String**> | Controller event block timestamp (seconds) (as embedded in the hub event payload) |  |
**event_signature** | Option<**String**> | Controller event signature hash (bytes32 hex) |  |
**abi_encoded_event_data** | Option<**String**> | Controller ABI-encoded event payload (0x hex) |  |
**select** | Option<**String**> | Filtering Columns |  |
**order** | Option<**String**> | Ordering |  |
**range** | Option<**String**> | Limiting and Pagination |  |
**range_unit** | Option<**String**> | Limiting and Pagination |  |[default to items]
**offset** | Option<**String**> | Limiting and Pagination |  |
**limit** | Option<**String**> | Limiting and Pagination |  |
**prefer** | Option<**String**> | Preference |  |

### Return type

[**Vec<models::HubControllerProcessed>**](hub_controller_processed.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.pgrst.object+json;nulls=stripped, application/vnd.pgrst.object+json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

