# \HubRealtorsApi

All URIs are relative to *http://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**hub_realtors_get**](HubRealtorsApi.md#hub_realtors_get) | **GET** /hub_realtors | Current realtor allowlist + realtor config (KV)



## hub_realtors_get

> Vec<models::HubRealtors> hub_realtors_get(realtor, valid_from_seq, valid_to_seq, allowed, min_fee_ppm, min_flat_fee, max_lease_duration_seconds, lease_rate_max_leases, lease_rate_window_seconds, select, order, range, range_unit, offset, limit, prefer)
Current realtor allowlist + realtor config (KV)

Realtors are addresses allowed to create leases on the hub (`UntronV3.createLease`).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realtor** | Option<**String**> | Realtor address (EVM) |  |
**valid_from_seq** | Option<**String**> | Event sequence at which this realtor snapshot became current |  |
**valid_to_seq** | Option<**String**> |  |  |
**allowed** | Option<**String**> | Whether this address is currently allowlisted to create leases |  |
**min_fee_ppm** | Option<**String**> | Realtor-specific minimum percentage fee floor (ppm) |  |
**min_flat_fee** | Option<**String**> | Realtor-specific minimum flat fee floor (USDT units) |  |
**max_lease_duration_seconds** | Option<**String**> | Realtor-specific maximum lease duration in seconds (NULL means no override) |  |
**lease_rate_max_leases** | Option<**String**> | Max lease creations allowed per window (NULL/0 means disabled) |  |
**lease_rate_window_seconds** | Option<**String**> | Window size (seconds) for lease creation rate limiting (NULL/0 means disabled) |  |
**select** | Option<**String**> | Filtering Columns |  |
**order** | Option<**String**> | Ordering |  |
**range** | Option<**String**> | Limiting and Pagination |  |
**range_unit** | Option<**String**> | Limiting and Pagination |  |[default to items]
**offset** | Option<**String**> | Limiting and Pagination |  |
**limit** | Option<**String**> | Limiting and Pagination |  |
**prefer** | Option<**String**> | Preference |  |

### Return type

[**Vec<models::HubRealtors>**](hub_realtors.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.pgrst.object+json;nulls=stripped, application/vnd.pgrst.object+json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

