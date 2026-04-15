# PostImageOcr200ResponseWordsResultInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**words** | Option<**String**> | 当前文字片段的识别结果。 | [optional]
**location** | Option<[**models::PostImageOcr200ResponseWordsResultInnerLocation**](post_image_ocr_200_response_words_result_inner_location.md)> |  | [optional]
**vertexes_location** | Option<[**Vec<models::PostImageOcr200ResponseWordsResultInnerVertexesLocationInner>**](post_image_ocr_200_response_words_result_inner_vertexes_location_inner.md)> | 当前文字片段的顶点坐标列表。只有在 `need_location=true` 时才会返回。 | [optional]
**score** | Option<**f64**> | 当前文字片段的置信度。部分结果会返回。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


