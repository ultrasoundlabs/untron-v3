# HubBridgers

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**target_token** | Option<**String**> | Token being bridged (EVM on hub chain)  Note: This is a Primary Key.<pk/> | [optional]
**target_chain_id** | Option<**i64**> | Destination EVM chainId  Note: This is a Primary Key.<pk/> | [optional]
**valid_from_seq** | Option<**i64**> | Event sequence at which this bridger route became current  Note: This is a Primary Key.<pk/> | [optional]
**valid_to_seq** | Option<**i64**> |  | [optional]
**bridger** | Option<**String**> | Bridger adapter contract address (EVM) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


