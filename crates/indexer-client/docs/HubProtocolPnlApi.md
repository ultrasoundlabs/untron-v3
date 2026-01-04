# \HubProtocolPnlApi

All URIs are relative to *http://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**hub_protocol_pnl_get**](HubProtocolPnlApi.md#hub_protocol_pnl_get) | **GET** /hub_protocol_pnl | Current protocol PnL snapshot (singleton)



## hub_protocol_pnl_get

> Vec<models::HubProtocolPnl> hub_protocol_pnl_get(valid_from_seq, valid_to_seq, pnl, delta, reason, select, order, range, range_unit, offset, limit, prefer)
Current protocol PnL snapshot (singleton)

Tracks fee revenue and rebalance/withdrawal deltas as emitted by the hub contract.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**valid_from_seq** | Option<**String**> | Event sequence at which this PnL snapshot became current |  |
**valid_to_seq** | Option<**String**> |  |  |
**pnl** | Option<**String**> | Current protocol PnL value (int256) |  |
**delta** | Option<**String**> | Delta applied at this event (int256) |  |
**reason** | Option<**String**> | PnL reason code (matches `UntronV3Index.PnlReason`) |  |
**select** | Option<**String**> | Filtering Columns |  |
**order** | Option<**String**> | Ordering |  |
**range** | Option<**String**> | Limiting and Pagination |  |
**range_unit** | Option<**String**> | Limiting and Pagination |  |[default to items]
**offset** | Option<**String**> | Limiting and Pagination |  |
**limit** | Option<**String**> | Limiting and Pagination |  |
**prefer** | Option<**String**> | Preference |  |

### Return type

[**Vec<models::HubProtocolPnl>**](hub_protocol_pnl.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.pgrst.object+json;nulls=stripped, application/vnd.pgrst.object+json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

