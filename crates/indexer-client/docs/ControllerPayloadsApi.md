# \ControllerPayloadsApi

All URIs are relative to *http://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**controller_payloads_get**](ControllerPayloadsApi.md#controller_payloads_get) | **GET** /controller_payloads | Current controller rebalancer payloads (KV)



## controller_payloads_get

> Vec<models::ControllerPayloads> controller_payloads_get(rebalancer, valid_from_seq, valid_to_seq, payload, select, order, range, range_unit, offset, limit, prefer)
Current controller rebalancer payloads (KV)

Payloads configure how USDT is bridged out of Tron for each rebalancer implementation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rebalancer** | Option<**String**> | Rebalancer address (Tron) |  |
**valid_from_seq** | Option<**String**> | Controller event sequence at which this payload became current |  |
**valid_to_seq** | Option<**String**> |  |  |
**payload** | Option<**String**> | Rebalancer-specific payload bytes (0x hex) used for delegatecall bridging |  |
**select** | Option<**String**> | Filtering Columns |  |
**order** | Option<**String**> | Ordering |  |
**range** | Option<**String**> | Limiting and Pagination |  |
**range_unit** | Option<**String**> | Limiting and Pagination |  |[default to items]
**offset** | Option<**String**> | Limiting and Pagination |  |
**limit** | Option<**String**> | Limiting and Pagination |  |
**prefer** | Option<**String**> | Preference |  |

### Return type

[**Vec<models::ControllerPayloads>**](controller_payloads.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.pgrst.object+json;nulls=stripped, application/vnd.pgrst.object+json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

