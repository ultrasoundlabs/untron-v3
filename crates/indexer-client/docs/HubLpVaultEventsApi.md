# \HubLpVaultEventsApi

All URIs are relative to *http://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**hub_lp_vault_events_get**](HubLpVaultEventsApi.md#hub_lp_vault_events_get) | **GET** /hub_lp_vault_events | Hub LP vault deposit/withdraw ledger (append-only)



## hub_lp_vault_events_get

> Vec<models::HubLpVaultEvents> hub_lp_vault_events_get(event_seq, kind, lp, amount, select, order, range, range_unit, offset, limit, prefer)
Hub LP vault deposit/withdraw ledger (append-only)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_seq** | Option<**String**> | Hub event sequence for this vault event |  |
**kind** | Option<**String**> | `deposit` or `withdraw` |  |
**lp** | Option<**String**> | LP address (EVM) |  |
**amount** | Option<**String**> | Amount deposited/withdrawn (uint256, USDT units) |  |
**select** | Option<**String**> | Filtering Columns |  |
**order** | Option<**String**> | Ordering |  |
**range** | Option<**String**> | Limiting and Pagination |  |
**range_unit** | Option<**String**> | Limiting and Pagination |  |[default to items]
**offset** | Option<**String**> | Limiting and Pagination |  |
**limit** | Option<**String**> | Limiting and Pagination |  |
**prefer** | Option<**String**> | Preference |  |

### Return type

[**Vec<models::HubLpVaultEvents>**](hub_lp_vault_events.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.pgrst.object+json;nulls=stripped, application/vnd.pgrst.object+json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

