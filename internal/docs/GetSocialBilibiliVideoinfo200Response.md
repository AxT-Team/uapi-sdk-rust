# GetSocialBilibiliVideoinfo200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bvid** | Option<**String**> | 稿件的BV号。 | [optional]
**aid** | Option<**f64**> | 稿件的AV号。 | [optional]
**videos** | Option<**f64**> | 稿件分P总数。如果是单P视频，则为1。 | [optional]
**tid** | Option<**f64**> | 视频所属的子分区 ID。 | [optional]
**tname** | Option<**String**> | 视频所属的子分区名称。 | [optional]
**copyright** | Option<**f64**> | 视频类型。1代表原创，2代表转载。 | [optional]
**pic** | Option<**String**> | 稿件封面图片的URL。这是一个可以直接在网页上展示的链接。 | [optional]
**title** | Option<**String**> | 稿件的标题。 | [optional]
**pubdate** | Option<**f64**> | 稿件发布时间的Unix时间戳（秒）。 | [optional]
**ctime** | Option<**f64**> | 用户投稿时间的Unix时间戳（秒）。 | [optional]
**desc** | Option<**String**> | 视频简介。可能会包含HTML换行符。 | [optional]
**desc_v2** | Option<[**Vec<models::GetSocialBilibiliVideoinfo200ResponseDescV2Inner>**](get_social_bilibili_videoinfo_200_response_desc_v2_inner.md)> | 结构化简介片段。 | [optional]
**state** | Option<**f64**> | 视频状态码。 | [optional]
**duration** | Option<**f64**> | 稿件总时长（所有分P累加），单位为秒。 | [optional]
**rights** | Option<[**models::GetSocialBilibiliVideoinfo200ResponseRights**](get_social_bilibili_videoinfo_200_response_rights.md)> |  | [optional]
**owner** | Option<[**models::GetSocialBilibiliVideoinfo200ResponseOwner**](get_social_bilibili_videoinfo_200_response_owner.md)> |  | [optional]
**stat** | Option<[**models::GetSocialBilibiliVideoinfo200ResponseStat**](get_social_bilibili_videoinfo_200_response_stat.md)> |  | [optional]
**dynamic** | Option<**String**> | 投稿时附带的动态文字。 | [optional]
**cid** | Option<**f64**> | 主分P的 CID（弹幕 ID）。 | [optional]
**dimension** | Option<[**models::GetSocialBilibiliVideoinfo200ResponseDimension**](get_social_bilibili_videoinfo_200_response_dimension.md)> |  | [optional]
**no_cache** | Option<**bool**> | 不缓存标记。 | [optional]
**pages** | Option<[**Vec<models::GetSocialBilibiliVideoinfo200ResponsePagesInner>**](get_social_bilibili_videoinfo_200_response_pages_inner.md)> | 视频分P列表。即使是单P视频，该数组也包含一个元素。 | [optional]
**subtitle** | Option<[**models::GetSocialBilibiliVideoinfo200ResponseSubtitle**](get_social_bilibili_videoinfo_200_response_subtitle.md)> |  | [optional]
**staff** | Option<[**Vec<models::GetSocialBilibiliVideoinfo200ResponseStaffInner>**](get_social_bilibili_videoinfo_200_response_staff_inner.md)> | 联合投稿成员列表。 | [optional]
**ugc_season** | Option<[**models::GetSocialBilibiliVideoinfo200ResponseUgcSeason**](get_social_bilibili_videoinfo_200_response_ugc_season.md)> |  | [optional]
**is_chargeable_season** | Option<**bool**> | 是否为付费合集。 | [optional]
**is_story** | Option<**bool**> | 是否为剧情类视频。 | [optional]
**honor_reply** | Option<[**models::GetSocialBilibiliVideoinfo200ResponseHonorReply**](get_social_bilibili_videoinfo_200_response_honor_reply.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


