# \HubProtocolConfigApi

All URIs are relative to *http://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**hub_protocol_config_get**](HubProtocolConfigApi.md#hub_protocol_config_get) | **GET** /hub_protocol_config | Current hub protocol configuration (singleton)



## hub_protocol_config_get

> Vec<models::HubProtocolConfig> hub_protocol_config_get(valid_from_seq, valid_to_seq, usdt, tron_usdt, tron_reader, floor_ppm, floor_flat_fee, max_lease_duration_seconds, lessee_rate_max_updates, lessee_rate_window_seconds, select, order, range, range_unit, offset, limit, prefer)
Current hub protocol configuration (singleton)

Derived from a set of hub config events (USDT, Tron reader, fee floors, rate limits).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**valid_from_seq** | Option<**String**> | Event sequence at which this config snapshot became current |  |
**valid_to_seq** | Option<**String**> |  |  |
**usdt** | Option<**String**> | EVM USDT accounting token address on the hub chain |  |
**tron_usdt** | Option<**String**> | Tron USDT TRC-20 contract address (base58) accepted by `preEntitle` |  |
**tron_reader** | Option<**String**> | Trusted Tron transaction reader address used to verify + decode Tron transactions |  |
**floor_ppm** | Option<**String**> | Protocol-wide minimum percentage fee floor (ppm) |  |
**floor_flat_fee** | Option<**String**> | Protocol-wide minimum flat fee floor (USDT units) |  |
**max_lease_duration_seconds** | Option<**String**> | Protocol-wide maximum lease duration in seconds (NULL/0 means disabled) |  |
**lessee_rate_max_updates** | Option<**String**> | Max payout config updates allowed per window per lessee (NULL/0 means disabled) |  |
**lessee_rate_window_seconds** | Option<**String**> | Window size (seconds) for payout config update rate limiting (NULL/0 means disabled) |  |
**select** | Option<**String**> | Filtering Columns |  |
**order** | Option<**String**> | Ordering |  |
**range** | Option<**String**> | Limiting and Pagination |  |
**range_unit** | Option<**String**> | Limiting and Pagination |  |[default to items]
**offset** | Option<**String**> | Limiting and Pagination |  |
**limit** | Option<**String**> | Limiting and Pagination |  |
**prefer** | Option<**String**> | Preference |  |

### Return type

[**Vec<models::HubProtocolConfig>**](hub_protocol_config.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.pgrst.object+json;nulls=stripped, application/vnd.pgrst.object+json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

