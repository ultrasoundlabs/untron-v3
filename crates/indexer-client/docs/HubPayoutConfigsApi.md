# \HubPayoutConfigsApi

All URIs are relative to *http://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**hub_payout_configs_get**](HubPayoutConfigsApi.md#hub_payout_configs_get) | **GET** /hub_payout_configs | Current payout configuration per lease (KV)



## hub_payout_configs_get

> Vec<models::HubPayoutConfigs> hub_payout_configs_get(lease_id, valid_from_seq, valid_to_seq, target_chain_id, target_token, beneficiary, select, order, range, range_unit, offset, limit, prefer)
Current payout configuration per lease (KV)

Defines destination chain, settlement token, and beneficiary for newly created claims under each lease.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lease_id** | Option<**String**> | Lease id this payout config applies to |  |
**valid_from_seq** | Option<**String**> | Event sequence at which this payout config became current |  |
**valid_to_seq** | Option<**String**> |  |  |
**target_chain_id** | Option<**String**> | Destination chainId for payouts created under this config |  |
**target_token** | Option<**String**> | Settlement token on the hub chain used for fills (USDT or swapped token) |  |
**beneficiary** | Option<**String**> | Recipient (EVM) for payouts / bridged delivery |  |
**select** | Option<**String**> | Filtering Columns |  |
**order** | Option<**String**> | Ordering |  |
**range** | Option<**String**> | Limiting and Pagination |  |
**range_unit** | Option<**String**> | Limiting and Pagination |  |[default to items]
**offset** | Option<**String**> | Limiting and Pagination |  |
**limit** | Option<**String**> | Limiting and Pagination |  |
**prefer** | Option<**String**> | Preference |  |

### Return type

[**Vec<models::HubPayoutConfigs>**](hub_payout_configs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.pgrst.object+json;nulls=stripped, application/vnd.pgrst.object+json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

