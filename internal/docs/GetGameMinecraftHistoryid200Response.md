# GetGameMinecraftHistoryid200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**query** | Option<**String**> | 【name 查询时返回】查询的用户名。 | [optional]
**count** | Option<**i32**> | 【name 查询时返回】匹配到的用户数量，为 0 时表示未找到。 | [optional]
**results** | Option<[**Vec<models::GetGameMinecraftHistoryid200ResponseResultsInner>**](get_game_minecraft_historyid_200_response_results_inner.md)> | 【name 查询时返回】匹配用户列表，包含当前用户名或曾用名匹配的所有玩家。 | [optional]
**id** | Option<**String**> | 【uuid 查询时返回】玩家当前的用户名。 | [optional]
**uuid** | Option<**String**> | 【uuid 查询时返回】被查询玩家的UUID（带连字符格式）。 | [optional]
**name_num** | Option<**i32**> | 【uuid 查询时返回】历史名称的总数（包含当前名称）。 | [optional]
**history** | Option<[**Vec<models::GetGameMinecraftHistoryid200ResponseHistoryInner>**](get_game_minecraft_historyid_200_response_history_inner.md)> | 【uuid 查询时返回】包含所有历史用户名的数组，按时间倒序排列。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


