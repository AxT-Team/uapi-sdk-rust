# GetSocialBilibiliLiveroom200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uid** | Option<**f64**> | 主播的用户ID (mid)。 | [optional]
**room_id** | Option<**f64**> | 直播间的真实房间号（长号）。 | [optional]
**short_id** | Option<**f64**> | 直播间的短号（靓号）。如果没有设置，则为0。 | [optional]
**attention** | Option<**f64**> | 主播的粉丝数（关注数量）。 | [optional]
**online** | Option<**f64**> | 直播间当前的人气值。注意这不是真实在线人数。 | [optional]
**live_status** | Option<**f64**> | 直播状态。0:未开播, 1:直播中, 2:轮播中。 | [optional]
**area_id** | Option<**f64**> | 分区ID。 | [optional]
**parent_area_name** | Option<**String**> | 父分区名称。 | [optional]
**area_name** | Option<**String**> | 子分区名称。 | [optional]
**background** | Option<**String**> | 直播间背景图的URL。 | [optional]
**title** | Option<**String**> | 当前直播间的标题。 | [optional]
**user_cover** | Option<**String**> | 用户设置的直播间封面URL。 | [optional]
**description** | Option<**String**> | 直播间公告或描述，支持换行符。 | [optional]
**live_time** | Option<**String**> | 本次直播开始的时间，格式为 `YYYY-MM-DD HH:mm:ss`。如果未开播，则为空字符串。 | [optional]
**tags** | Option<**String**> | 直播间设置的标签，以逗号分隔。 | [optional]
**hot_words** | Option<**Vec<String>**> | 直播间热词列表，通常用于弹幕互动。 | [optional]
**new_pendants** | Option<[**serde_json::Value**](.md)> | 主播佩戴的头像框、大航海等级等信息，结构可能比较复杂。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


