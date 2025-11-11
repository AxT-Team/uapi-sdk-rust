# \StatusApi

All URIs are relative to *https://uapis.cn/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_status_ratelimit**](StatusApi.md#get_status_ratelimit) | **GET** /status/ratelimit | 获取API限流器实时状态
[**get_status_usage**](StatusApi.md#get_status_usage) | **GET** /status/usage | 获取API端点使用统计



## get_status_ratelimit

> models::GetStatusRatelimit200Response get_status_ratelimit(authorization)
获取API限流器实时状态

想了解我们API的当前负载情况吗？这个接口为你提供了服务的“心电图”。  ## 功能概述 此接口返回我们后端自适应限流器的实时状态。你可以看到当前并发请求数、并发上限、系统负载、请求接受/拒绝数等核心指标。这对于监控API健康状况和性能表现至关重要。  > [!IMPORTANT] > 此接口为管理接口，需要提供有效的管理员级别API密钥才能访问。  ### 认证方式 请在请求头中添加 `Authorization: Bearer <你的API密钥>`。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Bearer类型的API密钥认证头。例如：`Bearer sk-xxx` | [required] |

### Return type

[**models::GetStatusRatelimit200Response**](get_status_ratelimit_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_status_usage

> models::GetStatusUsage200Response get_status_usage(path)
获取API端点使用统计

想知道哪个API接口最受欢迎吗？这个接口提供了详细的“账单”。  ## 功能概述 此接口用于获取每个API端点（Endpoint）的使用情况统计。你可以查询所有端点的列表，也可以通过 `path` 参数指定查询某一个特定端点。返回信息包括调用次数和平均处理时长

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | Option<**String**> | （可选）如果你想查询某个特定的端点，请提供它的路径，例如 '/api/v1/image/motou'。如果留空，则返回所有端点的统计列表。 |  |

### Return type

[**models::GetStatusUsage200Response**](get_status_usage_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

