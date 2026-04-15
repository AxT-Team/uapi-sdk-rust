# PostTextMarkdownToHtmlRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**text** | **String** | 原始 Markdown 字符串，最大不超过 1MB。 | 
**format** | Option<**String**> | 响应格式。传 `json` 时返回 JSON 包裹的 HTML 片段；传 `html` 时直接返回 `text/html`，并且响应内容会自动带完整的网页结构，适合浏览器预览或直接保存为网页文件。默认是 `json`。 | [optional][default to Json]
**sanitize** | Option<**bool**> | 是否开启安全模式，过滤掉用户输入中的风险脚本。默认是 `true`。 | [optional][default to true]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


