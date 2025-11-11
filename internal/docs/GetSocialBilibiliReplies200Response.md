# GetSocialBilibiliReplies200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**page** | Option<[**models::GetSocialBilibiliReplies200ResponsePage**](get_social_bilibili_replies_200_response_page.md)> |  | [optional]
**hots** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | 热门评论列表。结构与 `replies` 中的对象一致。如果当前页是第一页，且有热门评论，则此数组非空。 | [optional]
**replies** | Option<[**Vec<models::GetSocialBilibiliReplies200ResponseRepliesInner>**](get_social_bilibili_replies_200_response_replies_inner.md)> | 当前页的评论列表。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


