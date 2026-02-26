# GetMiscHotboard200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**list** | Option<[**Vec<models::GetMiscHotboard200ResponseListInner>**](get_misc_hotboard_200_response_list_inner.md)> | 热榜条目列表。 | [optional]
**r#type** | Option<**String**> |  | [optional]
**update_time** | Option<**String**> |  | [optional]
**snapshot_time** | Option<**i32**> | 时光机模式返回的快照实际时间戳（毫秒）。 | [optional]
**keyword** | Option<**String**> | 搜索模式返回的搜索关键词。 | [optional]
**count** | Option<**i32**> | 搜索模式返回的结果数量。 | [optional]
**results** | Option<[**Vec<models::GetMiscHotboard200ResponseResultsInner>**](get_misc_hotboard_200_response_results_inner.md)> | 搜索模式返回的结果数组。 | [optional]
**sources** | Option<**Vec<String>**> | 数据源列表模式返回的可用历史数据源数组。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


