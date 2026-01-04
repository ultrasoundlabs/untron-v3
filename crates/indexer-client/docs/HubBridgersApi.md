# \HubBridgersApi

All URIs are relative to *http://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**hub_bridgers_get**](HubBridgersApi.md#hub_bridgers_get) | **GET** /hub_bridgers | Current bridger routing table (KV)



## hub_bridgers_get

> Vec<models::HubBridgers> hub_bridgers_get(target_token, target_chain_id, valid_from_seq, valid_to_seq, bridger, select, order, range, range_unit, offset, limit, prefer)
Current bridger routing table (KV)

If a claim targets a different chain than the hub chain, the configured bridger is used to deliver funds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target_token** | Option<**String**> | Token being bridged (EVM on hub chain) |  |
**target_chain_id** | Option<**String**> | Destination EVM chainId |  |
**valid_from_seq** | Option<**String**> | Event sequence at which this bridger route became current |  |
**valid_to_seq** | Option<**String**> |  |  |
**bridger** | Option<**String**> | Bridger adapter contract address (EVM) |  |
**select** | Option<**String**> | Filtering Columns |  |
**order** | Option<**String**> | Ordering |  |
**range** | Option<**String**> | Limiting and Pagination |  |
**range_unit** | Option<**String**> | Limiting and Pagination |  |[default to items]
**offset** | Option<**String**> | Limiting and Pagination |  |
**limit** | Option<**String**> | Limiting and Pagination |  |
**prefer** | Option<**String**> | Preference |  |

### Return type

[**Vec<models::HubBridgers>**](hub_bridgers.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.pgrst.object+json;nulls=stripped, application/vnd.pgrst.object+json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

