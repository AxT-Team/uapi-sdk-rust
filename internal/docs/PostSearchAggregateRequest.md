# PostSearchAggregateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**query** | **String** | 搜索查询关键词，支持中英文 | 
**site** | Option<**String**> | 限制搜索特定网站，不需要 `site:` 前缀 | [optional]
**filetype** | Option<**String**> | 限制文件类型，不需要 `filetype:` 前缀。支持 pdf、doc、docx、ppt、pptx、xls、xlsx、txt 等 | [optional]
**fetch_full** | Option<**bool**> | 是否获取页面完整正文（会影响响应时间） | [optional][default to false]
**sort** | Option<**String**> | 排序方式 | [optional][default to Relevance]
**time_range** | Option<**String**> | 时间范围过滤 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


