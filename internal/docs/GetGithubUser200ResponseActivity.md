# GetGithubUser200ResponseActivity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**scope** | Option<**String**> | 活动统计范围，常见值为 all 或 organization。 | [optional]
**organization** | Option<**String**> | 按组织过滤时对应的组织登录名。 | [optional]
**from** | Option<**String**> | 统计开始日期。 | [optional]
**to** | Option<**String**> | 统计结束日期。 | [optional]
**total_contributions** | Option<**i32**> | 统计范围内的总贡献数。 | [optional]
**total_commit_contributions** | Option<**i32**> | Commit 贡献总数。 | [optional]
**total_issue_contributions** | Option<**i32**> | Issue 贡献总数。 | [optional]
**total_pull_request_contributions** | Option<**i32**> | Pull Request 贡献总数。 | [optional]
**total_pull_request_review_contributions** | Option<**i32**> | Pull Request Review 贡献总数。 | [optional]
**contribution_calendar** | Option<[**models::GetGithubUser200ResponseActivityContributionCalendar**](get_github_user_200_response_activity_contribution_calendar.md)> |  | [optional]
**timeline** | Option<[**Vec<models::GetGithubUser200ResponseActivityTimelineInner>**](get_github_user_200_response_activity_timeline_inner.md)> | 按月份聚合后的贡献时间线。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


