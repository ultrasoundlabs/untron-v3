# HubControllerTipUpdates

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_seq** | Option<**i32**> | Hub event sequence for this controller tip update record  Note: This is a Primary Key.<pk/> | [optional]
**previous_tip** | Option<**String**> | Controller tip that this hop links from | [optional]
**block_number** | Option<**i32**> | Controller event block number (as embedded in the hub event payload) | [optional]
**block_timestamp** | Option<**i32**> | Controller event block timestamp (seconds) (as embedded in the hub event payload) | [optional]
**event_signature** | Option<**String**> | Controller event signature hash (bytes32 hex) | [optional]
**abi_encoded_event_data** | Option<**String**> | Controller ABI-encoded event payload (0x hex) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


