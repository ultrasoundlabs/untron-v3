# \ControllerLpTokensWithdrawnApi

All URIs are relative to *http://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**controller_lp_tokens_withdrawn_get**](ControllerLpTokensWithdrawnApi.md#controller_lp_tokens_withdrawn_get) | **GET** /controller_lp_tokens_withdrawn | Controller LP token withdrawal ledger (append-only)



## controller_lp_tokens_withdrawn_get

> Vec<models::ControllerLpTokensWithdrawn> controller_lp_tokens_withdrawn_get(event_seq, token, amount, select, order, range, range_unit, offset, limit, prefer)
Controller LP token withdrawal ledger (append-only)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_seq** | Option<**String**> | Controller event sequence for this LP withdrawal |  |
**token** | Option<**String**> | Token withdrawn by LP (Tron address) |  |
**amount** | Option<**String**> | Amount withdrawn (uint256) |  |
**select** | Option<**String**> | Filtering Columns |  |
**order** | Option<**String**> | Ordering |  |
**range** | Option<**String**> | Limiting and Pagination |  |
**range_unit** | Option<**String**> | Limiting and Pagination |  |[default to items]
**offset** | Option<**String**> | Limiting and Pagination |  |
**limit** | Option<**String**> | Limiting and Pagination |  |
**prefer** | Option<**String**> | Preference |  |

### Return type

[**Vec<models::ControllerLpTokensWithdrawn>**](controller_lp_tokens_withdrawn.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.pgrst.object+json;nulls=stripped, application/vnd.pgrst.object+json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

