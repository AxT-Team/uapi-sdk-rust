# PostImageNsfw200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**nsfw_score** | Option<**f64**> | NSFW 内容的置信度分数，范围 0-1，越高表示越可能是敏感内容。 | [optional]
**normal_score** | Option<**f64**> | 正常内容的置信度分数，范围 0-1。 | [optional]
**is_nsfw** | Option<**bool**> | 是否判定为 NSFW 内容。 | [optional]
**label** | Option<**String**> | 内容标签，'nsfw' 或 'normal'。 | [optional]
**suggestion** | Option<**String**> | 处理建议：'pass'（通过）、'review'（人工复核）、'block'（拦截）。 | [optional]
**risk_level** | Option<**String**> | 风险等级：'low'、'medium'、'high'。 | [optional]
**confidence** | Option<**f64**> | 模型对当前判断的置信度。 | [optional]
**inference_time_ms** | Option<**f64**> | 模型推理耗时，单位毫秒。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


