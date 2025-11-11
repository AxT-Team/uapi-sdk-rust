# \TextApi

All URIs are relative to *https://uapis.cn/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_text_md5**](TextApi.md#get_text_md5) | **GET** /text/md5 | 计算文本的MD5哈希值(GET)
[**post_text_aes_decrypt**](TextApi.md#post_text_aes_decrypt) | **POST** /text/aes/decrypt | 使用AES算法解密文本
[**post_text_aes_encrypt**](TextApi.md#post_text_aes_encrypt) | **POST** /text/aes/encrypt | 使用AES算法加密文本
[**post_text_analyze**](TextApi.md#post_text_analyze) | **POST** /text/analyze | 多维度分析文本内容
[**post_text_base64_decode**](TextApi.md#post_text_base64_decode) | **POST** /text/base64/decode | 解码Base64编码的文本
[**post_text_base64_encode**](TextApi.md#post_text_base64_encode) | **POST** /text/base64/encode | 将文本进行Base64编码
[**post_text_md5**](TextApi.md#post_text_md5) | **POST** /text/md5 | 计算文本的MD5哈希值 (POST)
[**post_text_md5_verify**](TextApi.md#post_text_md5_verify) | **POST** /text/md5/verify | 校验MD5哈希值



## get_text_md5

> models::GetTextMd5200Response get_text_md5(text)
计算文本的MD5哈希值(GET)

一个快速计算文本 MD5 哈希值的工具，适用于短文本且不关心参数暴露的场景。  ## 功能概述 通过GET请求的查询参数传入文本，返回其32位小写的MD5哈希值。  > [!NOTE] > 对于较长或敏感的文本，我们推荐使用本接口的 POST 版本，以避免URL长度限制和参数暴露问题。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**text** | **String** | 需要计算哈希值的文本 | [required] |

### Return type

[**models::GetTextMd5200Response**](get_text_md5_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_text_aes_decrypt

> models::PostTextAesDecrypt200Response post_text_aes_decrypt(post_text_aes_decrypt_request)
使用AES算法解密文本

收到了用AES加密的密文？把它、密钥和随机数（nonce）交给我们，就能还原出原始内容。  ## 功能概述 这是一个标准的AES解密接口。你需要提供经过Base64编码的密文、加密时使用的密钥和nonce（随机数，通常为16字节字符串）。  > [!IMPORTANT] > **关于密钥 `key`** > 我们支持 AES-128, AES-192, 和 AES-256。请确保你提供的密钥 `key` 的长度（字节数）正好是 **16**、**24** 或 **32**，以分别对应这三种加密强度。 >  > **关于随机数 `nonce`** > 通常为16字节字符串，需与加密时一致。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_text_aes_decrypt_request** | [**PostTextAesDecryptRequest**](PostTextAesDecryptRequest.md) | 包含待解密文本 'text'、密钥 'key' 和随机数 'nonce' 的JSON对象 | [required] |

### Return type

[**models::PostTextAesDecrypt200Response**](post_text_aes_decrypt_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_text_aes_encrypt

> models::PostTextAesEncrypt200Response post_text_aes_encrypt(post_text_aes_encrypt_request)
使用AES算法加密文本

需要安全地传输或存储一些文本信息？AES加密是一个可靠的选择。  ## 功能概述 这是一个标准的AES加密接口。你提供需要加密的明文和密钥，我们返回经过Base64编码的密文。  > [!IMPORTANT] > **关于密钥 `key`** > 我们支持 AES-128, AES-192, 和 AES-256。请确保你提供的密钥 `key` 的长度（字节数）正好是 **16**、**24** 或 **32**，以分别对应这三种加密强度。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_text_aes_encrypt_request** | [**PostTextAesEncryptRequest**](PostTextAesEncryptRequest.md) | 包含待加密文本 'text' 和密钥 'key' 的JSON对象 | [required] |

### Return type

[**models::PostTextAesEncrypt200Response**](post_text_aes_encrypt_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_text_analyze

> models::PostTextAnalyze200Response post_text_analyze(post_text_analyze_request)
多维度分析文本内容

想知道一篇文章有多少字、多少个词、或者多少行？这个接口可以帮你快速统计。  ## 功能概述 你提供一段文本，我们会从多个维度进行分析，并返回其字符数、词数、句子数、段落数和行数。这对于文本编辑、内容管理等场景非常有用。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_text_analyze_request** | [**PostTextAnalyzeRequest**](PostTextAnalyzeRequest.md) | 包含待分析文本 'text' 的JSON对象 | [required] |

### Return type

[**models::PostTextAnalyze200Response**](post_text_analyze_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_text_base64_decode

> models::PostTextBase64Decode200Response post_text_base64_decode(post_text_base64_decode_request)
解码Base64编码的文本

这是一个简单实用的 Base64 解码工具。  ## 功能概述 你提供一个 Base64 编码的字符串，我们帮你解码成原始的 UTF-8 文本。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_text_base64_decode_request** | [**PostTextBase64DecodeRequest**](PostTextBase64DecodeRequest.md) | 包含待解码文本 'text' 的JSON对象 | [required] |

### Return type

[**models::PostTextBase64Decode200Response**](post_text_base64_decode_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_text_base64_encode

> models::PostTextBase64Encode200Response post_text_base64_encode(post_text_base64_encode_request)
将文本进行Base64编码

这是一个简单实用的 Base64 编码工具。  ## 功能概述 你提供一段原始文本，我们帮你转换成 Base64 编码的字符串。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_text_base64_encode_request** | [**PostTextBase64EncodeRequest**](PostTextBase64EncodeRequest.md) | 包含待编码文本 'text' 的JSON对象 | [required] |

### Return type

[**models::PostTextBase64Encode200Response**](post_text_base64_encode_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_text_md5

> models::GetTextMd5200Response post_text_md5(post_text_md5_request)
计算文本的MD5哈希值 (POST)

一个用于计算文本 MD5 哈希值的标准工具，推荐使用此版本。  ## 功能概述 通过POST请求的表单体传入文本，返回其32位小写的MD5哈希值。相比GET版本，此方法更适合处理较长或包含敏感信息的文本。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_text_md5_request** | [**PostTextMd5Request**](PostTextMd5Request.md) |  | [required] |

### Return type

[**models::GetTextMd5200Response**](get_text_md5_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_text_md5_verify

> models::PostTextMd5Verify200Response post_text_md5_verify(post_text_md5_verify_request)
校验MD5哈希值

下载了一个文件，想确认它在传输过程中有没有损坏？校验MD5值是最常用的方法。  ## 功能概述 你提供原始文本和一个MD5哈希值，我们帮你计算文本的哈希，并与你提供的哈希进行比对，告诉你它们是否匹配。这在文件完整性校验等场景下非常有用。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_text_md5_verify_request** | [**PostTextMd5VerifyRequest**](PostTextMd5VerifyRequest.md) | 包含待校验文本 'text' 和哈希值 'hash' 的JSON对象 | [required] |

### Return type

[**models::PostTextMd5Verify200Response**](post_text_md5_verify_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

