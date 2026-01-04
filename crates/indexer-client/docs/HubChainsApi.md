# \HubChainsApi

All URIs are relative to *http://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**hub_chains_get**](HubChainsApi.md#hub_chains_get) | **GET** /hub_chains | Current chain deprecation flags (KV)



## hub_chains_get

> Vec<models::HubChains> hub_chains_get(target_chain_id, valid_from_seq, valid_to_seq, deprecated, select, order, range, range_unit, offset, limit, prefer)
Current chain deprecation flags (KV)

Deprecated destination chains cannot be selected in new payout configs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target_chain_id** | Option<**String**> | Destination EVM chainId |  |
**valid_from_seq** | Option<**String**> | Event sequence at which this deprecation flag became current |  |
**valid_to_seq** | Option<**String**> |  |  |
**deprecated** | Option<**String**> | Whether this destination chain is deprecated for new payout configs |  |
**select** | Option<**String**> | Filtering Columns |  |
**order** | Option<**String**> | Ordering |  |
**range** | Option<**String**> | Limiting and Pagination |  |
**range_unit** | Option<**String**> | Limiting and Pagination |  |[default to items]
**offset** | Option<**String**> | Limiting and Pagination |  |
**limit** | Option<**String**> | Limiting and Pagination |  |
**prefer** | Option<**String**> | Preference |  |

### Return type

[**Vec<models::HubChains>**](hub_chains.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.pgrst.object+json;nulls=stripped, application/vnd.pgrst.object+json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

