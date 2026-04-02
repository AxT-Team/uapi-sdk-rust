# GetMiscHotboard200ResponseOneOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> |  | [optional]
**update_time** | Option<**String**> | 热榜更新时间。时光机无匹配快照时可能为空字符串。 | [optional]
**snapshot_time** | Option<**i32**> | 时光机模式返回的快照实际时间戳（毫秒）。当前热榜模式下通常不返回。 | [optional]
**list** | Option<[**Vec<models::GetMiscHotboard200ResponseOneOfListInner>**](get_misc_hotboard_200_response_oneOf_list_inner.md)> | 热榜条目列表。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


