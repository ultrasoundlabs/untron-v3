# HubClaims

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lease_id** | Option<[**serde_json::Number**](serde_json::Number.md)> | Lease id that produced this claim  Note: This is a Primary Key.<pk/> | [optional]
**claim_id** | Option<[**serde_json::Number**](serde_json::Number.md)> | Per-lease claim id (uint256, 0-indexed)  Note: This is a Primary Key.<pk/> | [optional]
**valid_from_seq** | Option<**i64**> | Event sequence at which this claim version became current  Note: This is a Primary Key.<pk/> | [optional]
**valid_to_seq** | Option<**i64**> |  | [optional]
**target_token** | Option<**String**> | Token used for settlement when filling this claim (EVM on hub chain) | [optional]
**queue_index** | Option<[**serde_json::Number**](serde_json::Number.md)> | Index in the FIFO queue (per target_token) where this claim was enqueued | [optional]
**amount_usdt** | Option<[**serde_json::Number**](serde_json::Number.md)> | USDT-denominated claim amount (uint256) used for accounting | [optional]
**target_chain_id** | Option<**i64**> | Destination chainId for payout (local if equals hub chainId) | [optional]
**beneficiary** | Option<**String**> | Beneficiary address (EVM) receiving payout | [optional]
**origin** | Option<**i64**> | Claim origin code (matches `UntronV3Index.ClaimOrigin`) | [optional]
**origin_id** | Option<**String**> | Origin identifier (txId for pre-entitle, receiver_salt for receiver pull, etc.) | [optional]
**origin_actor** | Option<**String**> | Origin actor (e.g. subjective pre-entitle sponsor; otherwise zero) | [optional]
**origin_token** | Option<**String**> | Origin token/address (Tron token for receiver pull; zero address otherwise) | [optional]
**origin_timestamp** | Option<**i64**> | Origin timestamp (seconds) (Tron block time or controller dump time; 0 if not applicable) | [optional]
**origin_raw_amount** | Option<[**serde_json::Number**](serde_json::Number.md)> | Raw amount before fees (USDT-equivalent units) | [optional]
**status** | Option<**String**> | Claim lifecycle status (`created` or `filled`) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


