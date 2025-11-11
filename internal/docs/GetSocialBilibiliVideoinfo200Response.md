# GetSocialBilibiliVideoinfo200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bvid** | Option<**String**> | 稿件的BV号。 | [optional]
**aid** | Option<**f64**> | 稿件的AV号。 | [optional]
**videos** | Option<**f64**> | 稿件分P总数。如果是单P视频，则为1。 | [optional]
**tname** | Option<**String**> | 视频所属的子分区名称。 | [optional]
**copyright** | Option<**f64**> | 视频类型。1代表原创，2代表转载。 | [optional]
**pic** | Option<**String**> | 稿件封面图片的URL。这是一个可以直接在网页上展示的链接。 | [optional]
**title** | Option<**String**> | 稿件的标题。 | [optional]
**pubdate** | Option<**f64**> | 稿件发布时间的Unix时间戳（秒）。 | [optional]
**ctime** | Option<**f64**> | 用户投稿时间的Unix时间戳（秒）。 | [optional]
**desc** | Option<**String**> | 视频简介。可能会包含HTML换行符。 | [optional]
**duration** | Option<**f64**> | 稿件总时长（所有分P累加），单位为秒。 | [optional]
**owner** | Option<[**models::GetSocialBilibiliVideoinfo200ResponseOwner**](get_social_bilibili_videoinfo_200_response_owner.md)> |  | [optional]
**stat** | Option<[**models::GetSocialBilibiliVideoinfo200ResponseStat**](get_social_bilibili_videoinfo_200_response_stat.md)> |  | [optional]
**pages** | Option<[**Vec<models::GetSocialBilibiliVideoinfo200ResponsePagesInner>**](get_social_bilibili_videoinfo_200_response_pages_inner.md)> | 视频分P列表。即使是单P视频，该数组也包含一个元素。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


