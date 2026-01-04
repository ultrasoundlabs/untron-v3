# \HubClaimsApi

All URIs are relative to *http://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**hub_claims_get**](HubClaimsApi.md#hub_claims_get) | **GET** /hub_claims | Current claim states (KV)



## hub_claims_get

> Vec<models::HubClaims> hub_claims_get(lease_id, claim_id, valid_from_seq, valid_to_seq, target_token, queue_index, amount_usdt, target_chain_id, beneficiary, origin, origin_id, origin_actor, origin_token, origin_timestamp, origin_raw_amount, status, select, order, range, range_unit, offset, limit, prefer)
Current claim states (KV)

Claims are created by proven Tron deposits / LP-sponsored subjective pre-entitlement / controller profit volume, and transition to `filled` when a filler settles them.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lease_id** | Option<**String**> | Lease id that produced this claim |  |
**claim_id** | Option<**String**> | Per-lease claim id (uint256, 0-indexed) |  |
**valid_from_seq** | Option<**String**> | Event sequence at which this claim version became current |  |
**valid_to_seq** | Option<**String**> |  |  |
**target_token** | Option<**String**> | Token used for settlement when filling this claim (EVM on hub chain) |  |
**queue_index** | Option<**String**> | Index in the FIFO queue (per target_token) where this claim was enqueued |  |
**amount_usdt** | Option<**String**> | USDT-denominated claim amount (uint256) used for accounting |  |
**target_chain_id** | Option<**String**> | Destination chainId for payout (local if equals hub chainId) |  |
**beneficiary** | Option<**String**> | Beneficiary address (EVM) receiving payout |  |
**origin** | Option<**String**> | Claim origin code (matches `UntronV3Index.ClaimOrigin`) |  |
**origin_id** | Option<**String**> | Origin identifier (txId for pre-entitle, receiver_salt for receiver pull, etc.) |  |
**origin_actor** | Option<**String**> | Origin actor (e.g. subjective pre-entitle sponsor; otherwise zero) |  |
**origin_token** | Option<**String**> | Origin token/address (Tron token for receiver pull; zero address otherwise) |  |
**origin_timestamp** | Option<**String**> | Origin timestamp (seconds) (Tron block time or controller dump time; 0 if not applicable) |  |
**origin_raw_amount** | Option<**String**> | Raw amount before fees (USDT-equivalent units) |  |
**status** | Option<**String**> | Claim lifecycle status (`created` or `filled`) |  |
**select** | Option<**String**> | Filtering Columns |  |
**order** | Option<**String**> | Ordering |  |
**range** | Option<**String**> | Limiting and Pagination |  |
**range_unit** | Option<**String**> | Limiting and Pagination |  |[default to items]
**offset** | Option<**String**> | Limiting and Pagination |  |
**limit** | Option<**String**> | Limiting and Pagination |  |
**prefer** | Option<**String**> | Preference |  |

### Return type

[**Vec<models::HubClaims>**](hub_claims.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.pgrst.object+json;nulls=stripped, application/vnd.pgrst.object+json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

