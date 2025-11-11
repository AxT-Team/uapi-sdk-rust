# GetGameMinecraftServerstatus200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | Option<**i32**> | 状态码，200代表成功。 | [optional]
**favicon_url** | Option<**String**> | 服务器图标的 Base64 Data URI。你可以直接在 `<img>` 标签的 `src` 属性中使用它。 | [optional]
**ip** | Option<**String**> | 服务器解析后的IP地址。 | [optional]
**max_players** | Option<**i32**> | 服务器配置的最大玩家容量。 | [optional]
**motd_clean** | Option<**String**> | 纯文本格式的服务器MOTD（每日消息），去除了所有颜色和格式代码。 | [optional]
**motd_html** | Option<**String**> | HTML格式的服务器MOTD，保留了颜色和样式，方便你在网页上直接渲染。 | [optional]
**online** | Option<**bool**> | 服务器当前是否在线。 | [optional]
**players** | Option<**i32**> | 当前在线的玩家数量。 | [optional]
**port** | Option<**i32**> | 服务器使用的端口。 | [optional]
**version** | Option<**String**> | 服务器报告的版本信息。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


