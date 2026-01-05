# HubProtocolConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**valid_from_seq** | Option<**i64**> | Event sequence at which this config snapshot became current  Note: This is a Primary Key.<pk/> | [optional]
**valid_to_seq** | Option<**i64**> |  | [optional]
**usdt** | Option<**String**> | EVM USDT accounting token address on the hub chain | [optional]
**tron_usdt** | Option<**String**> | Tron USDT TRC-20 contract address (base58) accepted by `preEntitle` | [optional]
**tron_reader** | Option<**String**> | Trusted Tron transaction reader address used to verify + decode Tron transactions | [optional]
**floor_ppm** | Option<**i64**> | Protocol-wide minimum percentage fee floor (ppm) | [optional]
**floor_flat_fee** | Option<[**serde_json::Number**](serde_json::Number.md)> | Protocol-wide minimum flat fee floor (USDT units) | [optional]
**max_lease_duration_seconds** | Option<**i64**> | Protocol-wide maximum lease duration in seconds (NULL/0 means disabled) | [optional]
**lessee_rate_max_updates** | Option<[**serde_json::Number**](serde_json::Number.md)> | Max payout config updates allowed per window per lessee (NULL/0 means disabled) | [optional]
**lessee_rate_window_seconds** | Option<[**serde_json::Number**](serde_json::Number.md)> | Window size (seconds) for payout config update rate limiting (NULL/0 means disabled) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


