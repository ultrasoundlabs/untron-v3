# ControllerTipProofs

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**block_number** | Option<**i64**> | Block number containing the controller `IsEventChainTipCalled` log | [optional]
**block_timestamp** | Option<**i64**> | Block timestamp (seconds since epoch) containing the log | [optional]
**block_hash** | Option<**String**> | Block hash of the log's block (bytes32 hex) | [optional]
**block_time** | Option<**String**> | Convenience timestamp (block_timestamp converted to timestamptz) | [optional]
**tx_hash** | Option<**String**> | Transaction hash of the transaction containing the log (bytes32 hex) | [optional]
**log_index** | Option<**i64**> | Log index within the transaction receipt (0-based) | [optional]
**caller** | Option<**String**> | Tron address that called `isEventChainTip(bytes32)` | [optional]
**proved_tip** | Option<**String**> | Hash-chain tip value that the caller asserted as the controller's current tip | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


