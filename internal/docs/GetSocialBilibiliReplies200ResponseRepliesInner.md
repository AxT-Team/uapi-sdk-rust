# GetSocialBilibiliReplies200ResponseRepliesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rpid** | Option<**f64**> | 评论的唯一ID (Reply ID)。 | [optional]
**oid** | Option<**f64**> | 评论区对象ID，即视频的aid。 | [optional]
**mid** | Option<**f64**> | 发表评论的用户的mid。 | [optional]
**root** | Option<**f64**> | 根评论的rpid。如果为0，表示这条评论是根评论。 | [optional]
**parent** | Option<**f64**> | 回复的父级评论的rpid。如果为0，表示是根评论。 | [optional]
**count** | Option<**f64**> | 这条评论下的回复（楼中楼）数量。 | [optional]
**ctime** | Option<**f64**> | 评论发送时间的Unix时间戳（秒）。 | [optional]
**like** | Option<**f64**> | 该评论获得的点赞数。 | [optional]
**member** | Option<[**models::GetSocialBilibiliReplies200ResponseRepliesInnerMember**](get_social_bilibili_replies_200_response_replies_inner_member.md)> |  | [optional]
**content** | Option<[**models::GetSocialBilibiliReplies200ResponseRepliesInnerContent**](get_social_bilibili_replies_200_response_replies_inner_content.md)> |  | [optional]
**replies** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | 楼中楼回复列表。结构与顶层评论对象一致，但通常此数组为空，需要单独请求。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


