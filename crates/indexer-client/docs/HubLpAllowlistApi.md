# \HubLpAllowlistApi

All URIs are relative to *http://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**hub_lp_allowlist_get**](HubLpAllowlistApi.md#hub_lp_allowlist_get) | **GET** /hub_lp_allowlist | Current LP allowlist (KV)



## hub_lp_allowlist_get

> Vec<models::HubLpAllowlist> hub_lp_allowlist_get(lp, valid_from_seq, valid_to_seq, allowed, select, order, range, range_unit, offset, limit, prefer)
Current LP allowlist (KV)

LPs must be allowlisted to deposit principal into the hub fast-fill vault.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lp** | Option<**String**> | LP address (EVM) |  |
**valid_from_seq** | Option<**String**> | Event sequence at which this allowlist entry became current |  |
**valid_to_seq** | Option<**String**> |  |  |
**allowed** | Option<**String**> | Whether this LP may deposit into the fast-fill vault (withdrawals are always allowed) |  |
**select** | Option<**String**> | Filtering Columns |  |
**order** | Option<**String**> | Ordering |  |
**range** | Option<**String**> | Limiting and Pagination |  |
**range_unit** | Option<**String**> | Limiting and Pagination |  |[default to items]
**offset** | Option<**String**> | Limiting and Pagination |  |
**limit** | Option<**String**> | Limiting and Pagination |  |
**prefer** | Option<**String**> | Preference |  |

### Return type

[**Vec<models::HubLpAllowlist>**](hub_lp_allowlist.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.pgrst.object+json;nulls=stripped, application/vnd.pgrst.object+json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

