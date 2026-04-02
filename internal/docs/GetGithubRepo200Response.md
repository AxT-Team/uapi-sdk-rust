# GetGithubRepo200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**full_name** | Option<**String**> | 仓库完整名称。 | [optional]
**description** | Option<**String**> | 仓库简介。 | [optional]
**homepage** | Option<**String**> | 仓库主页链接。 | [optional]
**default_branch** | Option<**String**> | 默认分支名称。 | [optional]
**primary_branch** | Option<**String**> | 主要分支名称（通常与默认分支一致）。 | [optional]
**default_branch_sha** | Option<**String**> | 默认分支最新提交的 SHA 哈希。 | [optional]
**visibility** | Option<**String**> | 仓库可见性，常见值为 `public` 或 `private`。 | [optional]
**archived** | Option<**bool**> | 仓库是否已归档。 | [optional]
**disabled** | Option<**bool**> | 仓库是否被禁用。 | [optional]
**fork** | Option<**bool**> | 是否为 Fork 仓库。 | [optional]
**language** | Option<**String**> | 主要语言。 | [optional]
**topics** | Option<**Vec<String>**> | 话题标签列表。 | [optional]
**license** | Option<**String**> | 开源许可证名称。 | [optional]
**stargazers** | Option<**i32**> | Star 数。 | [optional]
**forks** | Option<**i32**> | Fork 数。 | [optional]
**open_issues** | Option<**i32**> | 开放 Issue 数。 | [optional]
**watchers** | Option<**i32**> | 关注者数量（watchers/subscribers）。 | [optional]
**pushed_at** | Option<**String**> | 最后推送时间（ISO 8601）。 | [optional]
**created_at** | Option<**String**> | 创建时间（ISO 8601）。 | [optional]
**updated_at** | Option<**String**> | 更新时间（ISO 8601）。 | [optional]
**languages** | Option<**std::collections::HashMap<String, i32>**> | 语言统计（键为语言名，值为代码字节数）。 | [optional]
**collaborators** | Option<[**Vec<models::GetGithubRepo200ResponseCollaboratorsInner>**](get_github_repo_200_response_collaborators_inner.md)> | 协作者列表。受权限限制时可能为 null 或空数组。 | [optional]
**maintainers** | Option<[**Vec<models::GetGithubRepo200ResponseCollaboratorsInner>**](get_github_repo_200_response_collaborators_inner.md)> | 维护者列表（根据默认分支近期提交推断）。 | [optional]
**latest_release** | Option<[**models::GetGithubRepo200ResponseLatestRelease**](get_github_repo_200_response_latest_release.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


