# PostAiTranslate200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | Option<**i32**> |  | [optional]
**message** | Option<**String**> |  | [optional]
**is_batch** | Option<**bool**> | 标识是否为批量翻译请求。 | [optional]
**data** | Option<[**models::PostAiTranslate200ResponseData**](post_ai_translate_200_response_data.md)> |  | [optional]
**batch_data** | Option<[**Vec<models::PostAiTranslate200ResponseBatchDataInner>**](post_ai_translate_200_response_batch_data_inner.md)> | 批量翻译结果列表，仅在批量翻译时返回。 | [optional]
**batch_summary** | Option<[**models::PostAiTranslate200ResponseBatchSummary**](post_ai_translate_200_response_batch_summary.md)> |  | [optional]
**performance** | Option<[**models::PostAiTranslate200ResponsePerformance**](post_ai_translate_200_response_performance.md)> |  | [optional]
**quality_metrics** | Option<[**models::PostAiTranslate200ResponseQualityMetrics**](post_ai_translate_200_response_quality_metrics.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


