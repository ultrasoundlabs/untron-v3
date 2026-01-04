# \ControllerReceiversApi

All URIs are relative to *http://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**controller_receivers_get**](ControllerReceiversApi.md#controller_receivers_get) | **GET** /controller_receivers | Current mapping of receiver salts to deployed receiver addresses (KV)



## controller_receivers_get

> Vec<models::ControllerReceivers> controller_receivers_get(receiver_salt, valid_from_seq, valid_to_seq, receiver, select, order, range, range_unit, offset, limit, prefer)
Current mapping of receiver salts to deployed receiver addresses (KV)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**receiver_salt** | Option<**String**> | Receiver salt (bytes32) identifying the deterministic receiver |  |
**valid_from_seq** | Option<**String**> | Controller event sequence at which this receiver mapping became current |  |
**valid_to_seq** | Option<**String**> |  |  |
**receiver** | Option<**String**> | Receiver contract address on Tron (base58) |  |
**select** | Option<**String**> | Filtering Columns |  |
**order** | Option<**String**> | Ordering |  |
**range** | Option<**String**> | Limiting and Pagination |  |
**range_unit** | Option<**String**> | Limiting and Pagination |  |[default to items]
**offset** | Option<**String**> | Limiting and Pagination |  |
**limit** | Option<**String**> | Limiting and Pagination |  |
**prefer** | Option<**String**> | Preference |  |

### Return type

[**Vec<models::ControllerReceivers>**](controller_receivers.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.pgrst.object+json;nulls=stripped, application/vnd.pgrst.object+json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

