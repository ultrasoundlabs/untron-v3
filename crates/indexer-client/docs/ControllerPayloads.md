# ControllerPayloads

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rebalancer** | Option<**String**> | Rebalancer address (Tron)  Note: This is a Primary Key.<pk/> | [optional]
**valid_from_seq** | Option<**i64**> | Controller event sequence at which this payload became current  Note: This is a Primary Key.<pk/> | [optional]
**valid_to_seq** | Option<**i64**> |  | [optional]
**payload** | Option<**String**> | Rebalancer-specific payload bytes (0x hex) used for delegatecall bridging | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


