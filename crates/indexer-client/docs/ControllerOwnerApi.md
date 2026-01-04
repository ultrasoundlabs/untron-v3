# \ControllerOwnerApi

All URIs are relative to *http://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**controller_owner_get**](ControllerOwnerApi.md#controller_owner_get) | **GET** /controller_owner | Current controller owner (singleton)



## controller_owner_get

> Vec<models::ControllerOwner> controller_owner_get(valid_from_seq, valid_to_seq, owner, select, order, range, range_unit, offset, limit, prefer)
Current controller owner (singleton)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**valid_from_seq** | Option<**String**> | Controller event sequence at which this owner became current |  |
**valid_to_seq** | Option<**String**> |  |  |
**owner** | Option<**String**> | Controller owner (Tron address) |  |
**select** | Option<**String**> | Filtering Columns |  |
**order** | Option<**String**> | Ordering |  |
**range** | Option<**String**> | Limiting and Pagination |  |
**range_unit** | Option<**String**> | Limiting and Pagination |  |[default to items]
**offset** | Option<**String**> | Limiting and Pagination |  |
**limit** | Option<**String**> | Limiting and Pagination |  |
**prefer** | Option<**String**> | Preference |  |

### Return type

[**Vec<models::ControllerOwner>**](controller_owner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.pgrst.object+json;nulls=stripped, application/vnd.pgrst.object+json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

