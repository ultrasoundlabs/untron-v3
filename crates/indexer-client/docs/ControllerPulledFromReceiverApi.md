# \ControllerPulledFromReceiverApi

All URIs are relative to *http://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**controller_pulled_from_receiver_get**](ControllerPulledFromReceiverApi.md#controller_pulled_from_receiver_get) | **GET** /controller_pulled_from_receiver | Controller sweep ledger (append-only)



## controller_pulled_from_receiver_get

> Vec<models::ControllerPulledFromReceiver> controller_pulled_from_receiver_get(event_seq, receiver_salt, token, token_amount, exchange_rate, usdt_amount, select, order, range, range_unit, offset, limit, prefer)
Controller sweep ledger (append-only)

Records each receiver sweep and the computed USDT-equivalent amount.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_seq** | Option<**String**> | Controller event sequence for this receiver pull |  |
**receiver_salt** | Option<**String**> | Receiver salt identifying which deterministic receiver was swept |  |
**token** | Option<**String**> | Token pulled from receiver (Tron address) |  |
**token_amount** | Option<**String**> | Raw token amount pulled (uint256) |  |
**exchange_rate** | Option<**String**> | Exchange rate used (1e18-scaled); 1:1 for USDT sweeps |  |
**usdt_amount** | Option<**String**> | USDT-equivalent amount accounted for this pull (uint256) |  |
**select** | Option<**String**> | Filtering Columns |  |
**order** | Option<**String**> | Ordering |  |
**range** | Option<**String**> | Limiting and Pagination |  |
**range_unit** | Option<**String**> | Limiting and Pagination |  |[default to items]
**offset** | Option<**String**> | Limiting and Pagination |  |
**limit** | Option<**String**> | Limiting and Pagination |  |
**prefer** | Option<**String**> | Preference |  |

### Return type

[**Vec<models::ControllerPulledFromReceiver>**](controller_pulled_from_receiver.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.pgrst.object+json;nulls=stripped, application/vnd.pgrst.object+json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

