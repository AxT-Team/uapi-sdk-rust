# \ClipzyApi

All URIs are relative to *https://uapis.cn/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_clipzy_get**](ClipzyApi.md#get_clipzy_get) | **GET** /api/get | 步骤2 (方法一): 获取加密数据
[**get_clipzy_raw**](ClipzyApi.md#get_clipzy_raw) | **GET** /api/raw/{id} | 步骤2 (方法二): 获取原始文本
[**post_clipzy_store**](ClipzyApi.md#post_clipzy_store) | **POST** /api/store | 步骤1：上传加密数据



## get_clipzy_get

> models::GetClipzyGet200Response get_clipzy_get(id)
步骤2 (方法一): 获取加密数据

**此接口用于“最高安全等级”方法。**  您提供第一步中获得的ID，它会返回存储在服务器上的**加密数据**。您需要在自己的客户端中，使用您自己保管的密钥来解密它。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | 片段的唯一 ID。 | [required] |

### Return type

[**models::GetClipzyGet200Response**](get_clipzy_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_clipzy_raw

> String get_clipzy_raw(id, key)
步骤2 (方法二): 获取原始文本

**此接口用于“方便自动化”方法。**  您提供第一步获得的ID，并附上您自己保管的**解密密钥**作为 `key` 参数。服务器会直接为您解密，并返回纯文本内容。  > [!IMPORTANT] > 查看文档首页的 **cURL 示例**，了解此接口最典型的用法。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | 片段的唯一 ID。 | [required] |
**key** | **String** | 用于解密的 Base64 编码的 AES 密钥。 | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_clipzy_store

> models::PostClipzyStore200Response post_clipzy_store(post_clipzy_store_request)
步骤1：上传加密数据

这是所有流程的第一步。您的客户端应用需要先在本地准备好 **加密后的数据**，然后调用此接口进行上传。成功后，您会得到一个用于后续操作的唯一ID。  > [!NOTE] > 您发送给此接口的应该是**密文**，而不是原始文本。请参考文档首页的JavaScript示例来了解如何在客户端进行加密。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_clipzy_store_request** | [**PostClipzyStoreRequest**](PostClipzyStoreRequest.md) | 包含加密数据和可选的TTL。 | [required] |

### Return type

[**models::PostClipzyStore200Response**](post_clipzy_store_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

