# \ControllerUsdtRebalancedApi

All URIs are relative to *http://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**controller_usdt_rebalanced_get**](ControllerUsdtRebalancedApi.md#controller_usdt_rebalanced_get) | **GET** /controller_usdt_rebalanced | Controller USDT rebalance ledger (append-only)



## controller_usdt_rebalanced_get

> Vec<models::ControllerUsdtRebalanced> controller_usdt_rebalanced_get(event_seq, in_amount, out_amount, rebalancer, select, order, range, range_unit, offset, limit, prefer)
Controller USDT rebalance ledger (append-only)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_seq** | Option<**String**> | Controller event sequence for this rebalance |  |
**in_amount** | Option<**String**> | USDT amount bridged in (uint256) |  |
**out_amount** | Option<**String**> | Expected USDT amount out on destination (uint256) |  |
**rebalancer** | Option<**String**> | Rebalancer used (Tron address) |  |
**select** | Option<**String**> | Filtering Columns |  |
**order** | Option<**String**> | Ordering |  |
**range** | Option<**String**> | Limiting and Pagination |  |
**range_unit** | Option<**String**> | Limiting and Pagination |  |[default to items]
**offset** | Option<**String**> | Limiting and Pagination |  |
**limit** | Option<**String**> | Limiting and Pagination |  |
**prefer** | Option<**String**> | Preference |  |

### Return type

[**Vec<models::ControllerUsdtRebalanced>**](controller_usdt_rebalanced.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.pgrst.object+json;nulls=stripped, application/vnd.pgrst.object+json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

