# HubLeases

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lease_id** | Option<**f64**> | Global lease id (uint256)  Note: This is a Primary Key.<pk/> | [optional]
**valid_from_seq** | Option<**i32**> | Event sequence at which this lease became current  Note: This is a Primary Key.<pk/> | [optional]
**valid_to_seq** | Option<**i32**> |  | [optional]
**receiver_salt** | Option<**String**> | Receiver salt (bytes32) used to derive deterministic Tron receiver addresses | [optional]
**lease_number** | Option<**f64**> | Per-receiver lease index (0-based) for timeline ordering | [optional]
**realtor** | Option<**String**> | Realtor (EVM) that created this lease | [optional]
**lessee** | Option<**String**> | Lessee (EVM) who controls payout configuration | [optional]
**start_time** | Option<**i32**> | Lease start time on the hub chain (seconds) | [optional]
**nukeable_after** | Option<**i32**> | Earliest timestamp when the lease can be replaced by a new one for this receiver_salt | [optional]
**lease_fee_ppm** | Option<**i32**> | Percentage fee (ppm) applied to recognized raw volume | [optional]
**flat_fee** | Option<**f64**> | Flat fee (USDT units) applied after percentage fee | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


