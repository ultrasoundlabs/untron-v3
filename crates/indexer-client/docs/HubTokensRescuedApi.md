# \HubTokensRescuedApi

All URIs are relative to *http://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**hub_tokens_rescued_get**](HubTokensRescuedApi.md#hub_tokens_rescued_get) | **GET** /hub_tokens_rescued | Hub token rescue ledger (append-only)



## hub_tokens_rescued_get

> Vec<models::HubTokensRescued> hub_tokens_rescued_get(event_seq, token, amount, select, order, range, range_unit, offset, limit, prefer)
Hub token rescue ledger (append-only)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_seq** | Option<**String**> | Hub event sequence for this rescue |  |
**token** | Option<**String**> | Token rescued (EVM address on hub chain; must not be USDT) |  |
**amount** | Option<**String**> | Amount rescued (uint256) |  |
**select** | Option<**String**> | Filtering Columns |  |
**order** | Option<**String**> | Ordering |  |
**range** | Option<**String**> | Limiting and Pagination |  |
**range_unit** | Option<**String**> | Limiting and Pagination |  |[default to items]
**offset** | Option<**String**> | Limiting and Pagination |  |
**limit** | Option<**String**> | Limiting and Pagination |  |
**prefer** | Option<**String**> | Preference |  |

### Return type

[**Vec<models::HubTokensRescued>**](hub_tokens_rescued.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.pgrst.object+json;nulls=stripped, application/vnd.pgrst.object+json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

