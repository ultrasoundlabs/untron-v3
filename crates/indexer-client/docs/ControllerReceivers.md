# ControllerReceivers

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**receiver_salt** | Option<**String**> | Receiver salt (bytes32) identifying the deterministic receiver  Note: This is a Primary Key.<pk/> | [optional]
**valid_from_seq** | Option<**i64**> | Controller event sequence at which this receiver mapping became current  Note: This is a Primary Key.<pk/> | [optional]
**valid_to_seq** | Option<**i64**> |  | [optional]
**receiver** | Option<**String**> | Receiver contract address on Tron (base58) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


