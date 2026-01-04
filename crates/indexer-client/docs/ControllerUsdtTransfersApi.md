# \ControllerUsdtTransfersApi

All URIs are relative to *http://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**controller_usdt_transfers_get**](ControllerUsdtTransfersApi.md#controller_usdt_transfers_get) | **GET** /controller_usdt_transfers | Controller executor USDT transfer ledger (append-only)



## controller_usdt_transfers_get

> Vec<models::ControllerUsdtTransfers> controller_usdt_transfers_get(event_seq, recipient, amount, select, order, range, range_unit, offset, limit, prefer)
Controller executor USDT transfer ledger (append-only)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_seq** | Option<**String**> | Controller event sequence for this executor transfer |  |
**recipient** | Option<**String**> | Recipient of USDT from controller (Tron address) |  |
**amount** | Option<**String**> | Amount transferred (uint256) |  |
**select** | Option<**String**> | Filtering Columns |  |
**order** | Option<**String**> | Ordering |  |
**range** | Option<**String**> | Limiting and Pagination |  |
**range_unit** | Option<**String**> | Limiting and Pagination |  |[default to items]
**offset** | Option<**String**> | Limiting and Pagination |  |
**limit** | Option<**String**> | Limiting and Pagination |  |
**prefer** | Option<**String**> | Preference |  |

### Return type

[**Vec<models::ControllerUsdtTransfers>**](controller_usdt_transfers.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.pgrst.object+json;nulls=stripped, application/vnd.pgrst.object+json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

