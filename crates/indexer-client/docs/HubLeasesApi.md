# \HubLeasesApi

All URIs are relative to *http://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**hub_leases_get**](HubLeasesApi.md#hub_leases_get) | **GET** /hub_leases | Current lease registry (KV)



## hub_leases_get

> Vec<models::HubLeases> hub_leases_get(lease_id, valid_from_seq, valid_to_seq, receiver_salt, lease_number, realtor, lessee, start_time, nukeable_after, lease_fee_ppm, flat_fee, select, order, range, range_unit, offset, limit, prefer)
Current lease registry (KV)

Leases define who controls payouts (lessee), which receiver salt they apply to, and fee schedule parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lease_id** | Option<**String**> | Global lease id (uint256) |  |
**valid_from_seq** | Option<**String**> | Event sequence at which this lease became current |  |
**valid_to_seq** | Option<**String**> |  |  |
**receiver_salt** | Option<**String**> | Receiver salt (bytes32) used to derive deterministic Tron receiver addresses |  |
**lease_number** | Option<**String**> | Per-receiver lease index (0-based) for timeline ordering |  |
**realtor** | Option<**String**> | Realtor (EVM) that created this lease |  |
**lessee** | Option<**String**> | Lessee (EVM) who controls payout configuration |  |
**start_time** | Option<**String**> | Lease start time on the hub chain (seconds) |  |
**nukeable_after** | Option<**String**> | Earliest timestamp when the lease can be replaced by a new one for this receiver_salt |  |
**lease_fee_ppm** | Option<**String**> | Percentage fee (ppm) applied to recognized raw volume |  |
**flat_fee** | Option<**String**> | Flat fee (USDT units) applied after percentage fee |  |
**select** | Option<**String**> | Filtering Columns |  |
**order** | Option<**String**> | Ordering |  |
**range** | Option<**String**> | Limiting and Pagination |  |
**range_unit** | Option<**String**> | Limiting and Pagination |  |[default to items]
**offset** | Option<**String**> | Limiting and Pagination |  |
**limit** | Option<**String**> | Limiting and Pagination |  |
**prefer** | Option<**String**> | Preference |  |

### Return type

[**Vec<models::HubLeases>**](hub_leases.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.pgrst.object+json;nulls=stripped, application/vnd.pgrst.object+json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

