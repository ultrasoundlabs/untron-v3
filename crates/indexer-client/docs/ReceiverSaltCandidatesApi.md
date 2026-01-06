# \ReceiverSaltCandidatesApi

All URIs are relative to *http://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**receiver_salt_candidates_get**](ReceiverSaltCandidatesApi.md#receiver_salt_candidates_get) | **GET** /receiver_salt_candidates | Receiver salt candidates for realtor selection.



## receiver_salt_candidates_get

> Vec<models::ReceiverSaltCandidates> receiver_salt_candidates_get(receiver_salt, receiver, receiver_evm, balance_amount, has_balance, nukeable_after, is_free, select, order, range, range_unit, offset, limit, prefer)
Receiver salt candidates for realtor selection.

Joins: - `api.controller_receivers` (allowed salts) - `api.receiver_usdt_balances` (cached balance view) - latest hub lease by receiver_salt (for `nukeable_after`)  Computed fields: - `has_balance`: `balance_amount > 0` - `is_free`: receiver has no current lease or lease is nukeable (based on `nukeable_after <= now`)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**receiver_salt** | Option<**String**> |  |  |
**receiver** | Option<**String**> |  |  |
**receiver_evm** | Option<**String**> |  |  |
**balance_amount** | Option<**String**> |  |  |
**has_balance** | Option<**String**> |  |  |
**nukeable_after** | Option<**String**> |  |  |
**is_free** | Option<**String**> |  |  |
**select** | Option<**String**> | Filtering Columns |  |
**order** | Option<**String**> | Ordering |  |
**range** | Option<**String**> | Limiting and Pagination |  |
**range_unit** | Option<**String**> | Limiting and Pagination |  |[default to items]
**offset** | Option<**String**> | Limiting and Pagination |  |
**limit** | Option<**String**> | Limiting and Pagination |  |
**prefer** | Option<**String**> | Preference |  |

### Return type

[**Vec<models::ReceiverSaltCandidates>**](receiver_salt_candidates.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.pgrst.object+json;nulls=stripped, application/vnd.pgrst.object+json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

