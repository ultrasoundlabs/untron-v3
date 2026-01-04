# \ReceiverUsdtTransferActionabilityApi

All URIs are relative to *http://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**receiver_usdt_transfer_actionability_get**](ReceiverUsdtTransferActionabilityApi.md#receiver_usdt_transfer_actionability_get) | **GET** /receiver_usdt_transfer_actionability | Receiver USDT deposits + actionability hints (preEntitle vs pull)



## receiver_usdt_transfer_actionability_get

> Vec<models::ReceiverUsdtTransferActionability> receiver_usdt_transfer_actionability_get(chain_id, token, receiver_salt, sender, recipient, amount, block_number, block_timestamp, block_time, block_hash, tx_hash, log_index, claim_origin, claim_lease_id, claim_id, claim_status, claim_amount_usdt, expected_lease_id, last_pull_timestamp, preentitle_time_ok, recommended_action, select, order, range, range_unit, offset, limit, prefer)
Receiver USDT deposits + actionability hints (preEntitle vs pull)

For each canonical TRC-20 USDT transfer into a deterministic receiver, this view shows: - whether the hub has already accounted for it (matching claim origin_id == tx_hash), - the latest observed receiver pull timestamp for (receiver_salt, token), and - whether `preEntitle` is still time-eligible (`transfer_ts > last_pull_ts`).  `recommended_action` is a best-effort operator hint: - 'already_accounted' => hub claim exists (origin pre-entitle or subjective pre-entitle) - 'pre_entitle'        => no claim yet and pre-entitle timing is still allowed - 'pull'               => no claim yet and a later pull timestamp suggests pre-entitle would revert

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain_id** | Option<**String**> |  |  |
**token** | Option<**String**> |  |  |
**receiver_salt** | Option<**String**> |  |  |
**sender** | Option<**String**> |  |  |
**recipient** | Option<**String**> |  |  |
**amount** | Option<**String**> |  |  |
**block_number** | Option<**String**> |  |  |
**block_timestamp** | Option<**String**> |  |  |
**block_time** | Option<**String**> |  |  |
**block_hash** | Option<**String**> |  |  |
**tx_hash** | Option<**String**> |  |  |
**log_index** | Option<**String**> |  |  |
**claim_origin** | Option<**String**> |  |  |
**claim_lease_id** | Option<**String**> |  |  |
**claim_id** | Option<**String**> |  |  |
**claim_status** | Option<**String**> |  |  |
**claim_amount_usdt** | Option<**String**> |  |  |
**expected_lease_id** | Option<**String**> |  |  |
**last_pull_timestamp** | Option<**String**> |  |  |
**preentitle_time_ok** | Option<**String**> |  |  |
**recommended_action** | Option<**String**> |  |  |
**select** | Option<**String**> | Filtering Columns |  |
**order** | Option<**String**> | Ordering |  |
**range** | Option<**String**> | Limiting and Pagination |  |
**range_unit** | Option<**String**> | Limiting and Pagination |  |[default to items]
**offset** | Option<**String**> | Limiting and Pagination |  |
**limit** | Option<**String**> | Limiting and Pagination |  |
**prefer** | Option<**String**> | Preference |  |

### Return type

[**Vec<models::ReceiverUsdtTransferActionability>**](receiver_usdt_transfer_actionability.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.pgrst.object+json;nulls=stripped, application/vnd.pgrst.object+json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

