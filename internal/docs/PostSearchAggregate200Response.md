# PostSearchAggregate200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**query** | Option<**String**> | 执行的搜索查询 | [optional]
**total_results** | Option<**i32**> | 返回的搜索结果总数 | [optional]
**results** | Option<[**Vec<models::PostSearchAggregate200ResponseResultsInner>**](post_search_aggregate_200_response_results_inner.md)> | 搜索结果列表 | [optional]
**sources** | Option<[**Vec<models::PostSearchAggregate200ResponseSourcesInner>**](post_search_aggregate_200_response_sources_inner.md)> | 本次请求实际命中的搜索引擎信息 | [optional]
**process_time_ms** | Option<**i32**> | 本次请求总耗时（毫秒） | [optional]
**metadata** | Option<[**models::PostSearchAggregate200ResponseMetadata**](post_search_aggregate_200_response_metadata.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


