# HubRealtors

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**realtor** | Option<**String**> | Realtor address (EVM)  Note: This is a Primary Key.<pk/> | [optional]
**valid_from_seq** | Option<**i64**> | Event sequence at which this realtor snapshot became current  Note: This is a Primary Key.<pk/> | [optional]
**valid_to_seq** | Option<**i64**> |  | [optional]
**allowed** | Option<**bool**> | Whether this address is currently allowlisted to create leases | [optional]
**min_fee_ppm** | Option<**i64**> | Realtor-specific minimum percentage fee floor (ppm) | [optional]
**min_flat_fee** | Option<[**serde_json::Number**](serde_json::Number.md)> | Realtor-specific minimum flat fee floor (USDT units) | [optional]
**max_lease_duration_seconds** | Option<**i64**> | Realtor-specific maximum lease duration in seconds (NULL means no override) | [optional]
**lease_rate_max_leases** | Option<[**serde_json::Number**](serde_json::Number.md)> | Max lease creations allowed per window (NULL/0 means disabled) | [optional]
**lease_rate_window_seconds** | Option<[**serde_json::Number**](serde_json::Number.md)> | Window size (seconds) for lease creation rate limiting (NULL/0 means disabled) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


