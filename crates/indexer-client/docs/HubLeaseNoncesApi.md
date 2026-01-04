# \HubLeaseNoncesApi

All URIs are relative to *http://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**hub_lease_nonces_get**](HubLeaseNoncesApi.md#hub_lease_nonces_get) | **GET** /hub_lease_nonces | Current per-lease nonces (KV)



## hub_lease_nonces_get

> Vec<models::HubLeaseNonces> hub_lease_nonces_get(lease_id, valid_from_seq, valid_to_seq, nonce, select, order, range, range_unit, offset, limit, prefer)
Current per-lease nonces (KV)

Used for replay protection on EIP-712 signature-based payout config updates.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lease_id** | Option<**String**> | Lease id |  |
**valid_from_seq** | Option<**String**> | Event sequence at which this nonce became current |  |
**valid_to_seq** | Option<**String**> |  |  |
**nonce** | Option<**String**> | Current nonce value used for EIP-712 signature replay protection |  |
**select** | Option<**String**> | Filtering Columns |  |
**order** | Option<**String**> | Ordering |  |
**range** | Option<**String**> | Limiting and Pagination |  |
**range_unit** | Option<**String**> | Limiting and Pagination |  |[default to items]
**offset** | Option<**String**> | Limiting and Pagination |  |
**limit** | Option<**String**> | Limiting and Pagination |  |
**prefer** | Option<**String**> | Preference |  |

### Return type

[**Vec<models::HubLeaseNonces>**](hub_lease_nonces.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.pgrst.object+json;nulls=stripped, application/vnd.pgrst.object+json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

