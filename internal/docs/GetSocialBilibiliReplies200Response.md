# GetSocialBilibiliReplies200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**page** | Option<[**models::GetSocialBilibiliReplies200ResponsePage**](get_social_bilibili_replies_200_response_page.md)> |  | [optional]
**config** | Option<[**serde_json::Value**](.md)> | 评论区配置。不同视频或不同权限下可能为 null。 | [optional]
**hots** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | 热门评论列表。结构与 `replies` 中的对象一致。如果当前页是第一页，且有热门评论，则此数组非空。 | [optional]
**replies** | Option<[**Vec<models::GetSocialBilibiliReplies200ResponseRepliesInner>**](get_social_bilibili_replies_200_response_replies_inner.md)> | 当前页的评论列表。 | [optional]
**upper** | Option<[**serde_json::Value**](.md)> | UP 主相关信息。无数据时为 null。 | [optional]
**top** | Option<[**serde_json::Value**](.md)> | 置顶评论信息。没有置顶评论时为 null。 | [optional]
**notice** | Option<[**serde_json::Value**](.md)> | 评论区公告信息。没有公告时为 null。 | [optional]
**vote** | Option<**f64**> | 评论区投票相关状态值。没有投票时通常为 0。 | [optional]
**folder** | Option<[**serde_json::Value**](.md)> | 评论折叠相关信息。没有数据时为 null。 | [optional]
**control** | Option<[**serde_json::Value**](.md)> | 评论区控制信息。没有数据时为 null。 | [optional]
**cursor** | Option<[**serde_json::Value**](.md)> | 游标翻页信息。部分场景下为 null。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


