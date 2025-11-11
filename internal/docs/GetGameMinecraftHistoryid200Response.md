# GetGameMinecraftHistoryid200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | Option<**i32**> | 状态码，200代表成功。 | [optional]
**history** | Option<[**Vec<models::GetGameMinecraftHistoryid200ResponseHistoryInner>**](get_game_minecraft_historyid_200_response_history_inner.md)> | 一个包含所有历史用户名的数组，按时间倒序排列。 | [optional]
**id** | Option<**String**> | 玩家当前的用户名。 | [optional]
**name_num** | Option<**i32**> | 历史名称的总数（包含当前名称）。 | [optional]
**uuid** | Option<**String**> | 被查询玩家的32位无破折号UUID。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


