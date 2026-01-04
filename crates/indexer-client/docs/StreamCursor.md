# StreamCursor

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**stream** | Option<**String**> | Stream name (`hub` or `controller`)  Note: This is a Primary Key.<pk/> | [optional]
**applied_through_seq** | Option<**i32**> | Highest contiguous canonical event sequence already applied to derived tables | [optional]
**tip** | Option<**String**> | Expected `prev_tip` for the next event to apply (hash-chain continuity check) | [optional]
**updated_at** | Option<**String**> | Timestamp when the cursor last advanced or rolled back | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


