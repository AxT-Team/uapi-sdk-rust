# PostImageOcr200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**text** | Option<**String**> | 按阅读顺序拼接后的识别文本。 | [optional]
**plain_text** | Option<**String**> | 纯文本结果，适合做搜索、索引或直接展示。 | [optional]
**markdown** | Option<**String**> | 根据图片中的标题、段落和表格整理出的 Markdown 文本。只有在 `return_markdown=true` 时才会返回。 | [optional]
**words_result** | Option<[**Vec<models::PostImageOcr200ResponseWordsResultInner>**](post_image_ocr_200_response_words_result_inner.md)> | 逐段文字结果。适合做高亮、框选和逐项解析。 | [optional]
**words_result_num** | Option<**i32**> | 识别出的文字片段数量。 | [optional]
**need_location** | Option<**bool**> | 本次响应是否包含坐标信息。 | [optional]
**timing** | Option<[**serde_json::Value**](.md)> | 耗时拆分信息，适合做性能统计或排查。 | [optional]
**summary** | Option<[**serde_json::Value**](.md)> | 识别结果的统计摘要。 | [optional]
**image** | Option<[**serde_json::Value**](.md)> | 图片本身的基础信息。 | [optional]
**lines** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | 按行组织的详细识别结果。 | [optional]
**blocks** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | 按块组织的详细识别结果。 | [optional]
**pages** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | 按页组织的详细识别结果。 | [optional]
**raw** | Option<[**serde_json::Value**](.md)> | 补充识别结果对象，适合需要继续解析更多细节字段的场景。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


