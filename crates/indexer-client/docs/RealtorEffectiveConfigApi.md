# \RealtorEffectiveConfigApi

All URIs are relative to *http://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**realtor_effective_config_get**](RealtorEffectiveConfigApi.md#realtor_effective_config_get) | **GET** /realtor_effective_config | Realtor effective config (protocol floors + realtor overrides + rate remaining).



## realtor_effective_config_get

> Vec<models::RealtorEffectiveConfig> realtor_effective_config_get(realtor, allowed, min_fee_ppm, min_flat_fee, max_duration_seconds, lease_rate_max_leases, lease_rate_window_seconds, lease_rate_remaining, select, order, range, range_unit, offset, limit, prefer)
Realtor effective config (protocol floors + realtor overrides + rate remaining).

This view merges the protocol-wide floor limits with the current realtor row and returns: - `allowed` - `min_fee_ppm`, `min_flat_fee`, `max_duration_seconds` (effective minima/maxima) - `lease_rate_*` and `lease_rate_remaining` (best-effort, computed from current leases within the window)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realtor** | Option<**String**> |  |  |
**allowed** | Option<**String**> |  |  |
**min_fee_ppm** | Option<**String**> |  |  |
**min_flat_fee** | Option<**String**> |  |  |
**max_duration_seconds** | Option<**String**> |  |  |
**lease_rate_max_leases** | Option<**String**> |  |  |
**lease_rate_window_seconds** | Option<**String**> |  |  |
**lease_rate_remaining** | Option<**String**> |  |  |
**select** | Option<**String**> | Filtering Columns |  |
**order** | Option<**String**> | Ordering |  |
**range** | Option<**String**> | Limiting and Pagination |  |
**range_unit** | Option<**String**> | Limiting and Pagination |  |[default to items]
**offset** | Option<**String**> | Limiting and Pagination |  |
**limit** | Option<**String**> | Limiting and Pagination |  |
**prefer** | Option<**String**> | Preference |  |

### Return type

[**Vec<models::RealtorEffectiveConfig>**](realtor_effective_config.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.pgrst.object+json;nulls=stripped, application/vnd.pgrst.object+json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

