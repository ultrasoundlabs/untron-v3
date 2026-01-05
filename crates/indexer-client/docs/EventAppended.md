# EventAppended

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**stream** | Option<**String**> | Which stream emitted this EventAppended log (`hub` or `controller`) | [optional]
**event_seq** | Option<**i64**> | Monotonic sequence number in the stream's onchain event hash-chain | [optional]
**prev_tip** | Option<**String**> | Previous onchain event hash-chain tip before this event | [optional]
**new_tip** | Option<**String**> | New onchain event hash-chain tip after this event | [optional]
**event_signature** | Option<**String**> | Keccak256 hash of the semantic event signature string (bytes32 hex) | [optional]
**abi_encoded_event_data** | Option<**String**> | Exact ABI-encoded event payload bytes that were hashed onchain (0x hex) | [optional]
**event_type** | Option<**String**> | Worker-decoded semantic event name (e.g. `LeaseCreated`, `ClaimCreated`) | [optional]
**args** | Option<[**serde_json::Value**](.md)> | Worker-decoded event arguments as JSON (snake_case keys; values as strings/hex) | [optional]
**block_number** | Option<**i64**> | Block number containing the EventAppended log | [optional]
**block_timestamp** | Option<**i64**> | Block timestamp (seconds since epoch) containing the EventAppended log | [optional]
**block_time** | Option<**String**> | Convenience timestamp (block_timestamp converted to timestamptz) | [optional]
**block_hash** | Option<**String**> | Block hash of the log's block (bytes32 hex) | [optional]
**tx_hash** | Option<**String**> | Transaction hash of the log's transaction (bytes32 hex) | [optional]
**log_index** | Option<**i64**> | Log index within the transaction receipt (0-based) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


