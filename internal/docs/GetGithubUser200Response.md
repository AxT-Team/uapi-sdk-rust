# GetGithubUser200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**login** | Option<**String**> | GitHub 登录名。 | [optional]
**name** | Option<**String**> | 用户公开显示的名称。 | [optional]
**bio** | Option<**String**> | 用户个人简介。 | [optional]
**company** | Option<**String**> | 用户填写的公司或组织信息。 | [optional]
**location** | Option<**String**> | 用户公开展示的地理位置。 | [optional]
**blog** | Option<**String**> | 用户填写的网站或博客地址。 | [optional]
**twitter_username** | Option<**String**> | 用户绑定的 X（Twitter）用户名。 | [optional]
**email** | Option<**String**> | 用户公开可见的邮箱地址。 | [optional]
**html_url** | Option<**String**> | GitHub 个人主页链接。 | [optional]
**avatar_url** | Option<**String**> | 用户头像图片链接。 | [optional]
**r#type** | Option<**String**> | 账号类型，常见值为 User 或 Organization。 | [optional]
**public_repos** | Option<**i32**> | 公开仓库数量。 | [optional]
**public_gists** | Option<**i32**> | 公开 Gist 数量。 | [optional]
**followers** | Option<**i32**> | 关注该用户的人数。 | [optional]
**following** | Option<**i32**> | 该用户正在关注的人数。 | [optional]
**created_at** | Option<**String**> | GitHub 账号创建时间（ISO 8601）。 | [optional]
**updated_at** | Option<**String**> | 用户资料最近更新时间（ISO 8601）。 | [optional]
**organizations** | Option<[**Vec<models::GetGithubUser200ResponseOrganizationsInner>**](get_github_user_200_response_organizations_inner.md)> | 用户公开加入的组织列表 | [optional]
**activity** | Option<[**models::GetGithubUser200ResponseActivity**](get_github_user_200_response_activity.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


