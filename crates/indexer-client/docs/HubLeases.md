# HubLeases

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lease_id** | Option<[**serde_json::Number**](serde_json::Number.md)> | Global lease id (uint256)  Note: This is a Primary Key.<pk/> | [optional]
**valid_from_seq** | Option<**i64**> | Event sequence at which this lease became current  Note: This is a Primary Key.<pk/> | [optional]
**valid_to_seq** | Option<**i64**> |  | [optional]
**receiver_salt** | Option<**String**> | Receiver salt (bytes32) used to derive deterministic Tron receiver addresses | [optional]
**lease_number** | Option<[**serde_json::Number**](serde_json::Number.md)> | Per-receiver lease index (0-based) for timeline ordering | [optional]
**realtor** | Option<**String**> | Realtor (EVM) that created this lease | [optional]
**lessee** | Option<**String**> | Lessee (EVM) who controls payout configuration | [optional]
**start_time** | Option<**i64**> | Lease start time on the hub chain (seconds) | [optional]
**nukeable_after** | Option<**i64**> | Earliest timestamp when the lease can be replaced by a new one for this receiver_salt | [optional]
**lease_fee_ppm** | Option<**i64**> | Percentage fee (ppm) applied to recognized raw volume | [optional]
**flat_fee** | Option<[**serde_json::Number**](serde_json::Number.md)> | Flat fee (USDT units) applied after percentage fee | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


