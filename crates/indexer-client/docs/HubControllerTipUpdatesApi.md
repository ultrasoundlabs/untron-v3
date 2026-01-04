# \HubControllerTipUpdatesApi

All URIs are relative to *http://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**hub_controller_tip_updates_get**](HubControllerTipUpdatesApi.md#hub_controller_tip_updates_get) | **GET** /hub_controller_tip_updates | Hub-side controller tip update ledger (append-only)



## hub_controller_tip_updates_get

> Vec<models::HubControllerTipUpdates> hub_controller_tip_updates_get(event_seq, previous_tip, block_number, block_timestamp, event_signature, abi_encoded_event_data, select, order, range, range_unit, offset, limit, prefer)
Hub-side controller tip update ledger (append-only)

Records the raw controller event bytes that were hash-linked into the controller tip as seen by the hub.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_seq** | Option<**String**> | Hub event sequence for this controller tip update record |  |
**previous_tip** | Option<**String**> | Controller tip that this hop links from |  |
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

[**Vec<models::HubControllerTipUpdates>**](hub_controller_tip_updates.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.pgrst.object+json;nulls=stripped, application/vnd.pgrst.object+json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

