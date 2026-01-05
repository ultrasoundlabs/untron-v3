# HubControllerProcessed

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_seq** | Option<**i64**> | Hub event sequence for this controller processing record  Note: This is a Primary Key.<pk/> | [optional]
**event_index** | Option<[**serde_json::Number**](serde_json::Number.md)> | Index within the hub's controller event queue that was processed | [optional]
**block_number** | Option<**i64**> | Controller event block number (as embedded in the hub event payload) | [optional]
**block_timestamp** | Option<**i64**> | Controller event block timestamp (seconds) (as embedded in the hub event payload) | [optional]
**event_signature** | Option<**String**> | Controller event signature hash (bytes32 hex) | [optional]
**abi_encoded_event_data** | Option<**String**> | Controller ABI-encoded event payload (0x hex) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


