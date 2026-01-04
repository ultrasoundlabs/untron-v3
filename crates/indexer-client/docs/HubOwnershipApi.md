# \HubOwnershipApi

All URIs are relative to *http://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**hub_ownership_get**](HubOwnershipApi.md#hub_ownership_get) | **GET** /hub_ownership | Current hub owner (singleton)



## hub_ownership_get

> Vec<models::HubOwnership> hub_ownership_get(valid_from_seq, old_owner, new_owner, select, order, range, range_unit, offset, limit, prefer)
Current hub owner (singleton)

Derived from `OwnershipTransferred` events emitted via `UntronV3Index`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**valid_from_seq** | Option<**String**> | Event sequence at which this owner transition became current |  |
**old_owner** | Option<**String**> | Previous hub owner (EVM address) |  |
**new_owner** | Option<**String**> | New hub owner (EVM address) |  |
**select** | Option<**String**> | Filtering Columns |  |
**order** | Option<**String**> | Ordering |  |
**range** | Option<**String**> | Limiting and Pagination |  |
**range_unit** | Option<**String**> | Limiting and Pagination |  |[default to items]
**offset** | Option<**String**> | Limiting and Pagination |  |
**limit** | Option<**String**> | Limiting and Pagination |  |
**prefer** | Option<**String**> | Preference |  |

### Return type

[**Vec<models::HubOwnership>**](hub_ownership.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.pgrst.object+json;nulls=stripped, application/vnd.pgrst.object+json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

