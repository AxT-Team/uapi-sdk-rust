# PostSearchAggregate200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**query** | Option<**String**> | 实际执行的搜索查询 | [optional]
**total_results** | Option<**i32**> | 搜索结果总数 | [optional]
**results** | Option<[**Vec<models::PostSearchAggregate200ResponseResultsInner>**](post_search_aggregate_200_response_results_inner.md)> | 搜索结果列表 | [optional]
**sources** | Option<[**Vec<models::PostSearchAggregate200ResponseSourcesInner>**](post_search_aggregate_200_response_sources_inner.md)> | 各搜索引擎的结果数量统计 | [optional]
**process_time_ms** | Option<**i32**> | 处理耗时（毫秒） | [optional]
**cached** | Option<**bool**> | 结果是否来自缓存 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


