# HubProtocolPnl

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**valid_from_seq** | Option<**i32**> | Event sequence at which this PnL snapshot became current  Note: This is a Primary Key.<pk/> | [optional]
**valid_to_seq** | Option<**i32**> |  | [optional]
**pnl** | Option<**f64**> | Current protocol PnL value (int256) | [optional]
**delta** | Option<**f64**> | Delta applied at this event (int256) | [optional]
**reason** | Option<**i32**> | PnL reason code (matches `UntronV3Index.PnlReason`) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


