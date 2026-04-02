# \TranslateApi

All URIs are relative to *https://uapis.cn/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_ai_translate_languages**](TranslateApi.md#get_ai_translate_languages) | **GET** /ai/translate/languages | AI翻译配置
[**post_ai_translate**](TranslateApi.md#post_ai_translate) | **POST** /ai/translate | AI智能翻译
[**post_translate_stream**](TranslateApi.md#post_translate_stream) | **POST** /translate/stream | 流式翻译（中英互译）
[**post_translate_text**](TranslateApi.md#post_translate_text) | **POST** /translate/text | 翻译



## get_ai_translate_languages

> models::GetAiTranslateLanguages200Response get_ai_translate_languages()
AI翻译配置

获取AI智能翻译服务支持的完整语言列表、翻译风格选项、上下文场景选项以及性能指标信息。

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetAiTranslateLanguages200Response**](get_ai_translate_languages_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_ai_translate

> models::PostAiTranslate200Response post_ai_translate(target_lang, post_ai_translate_request)
AI智能翻译

这是一个商业级的AI智能翻译服务，采用最新的神经网络翻译技术和大语言模型，提供远超传统机器翻译的质量。  ## 功能概述  - **单文本翻译**: 专注处理单条文本翻译，适合需要高质量译文的业务场景。 - **多风格适配**: 提供随意口语化、专业商务、学术正式、文学艺术四种翻译风格，能够根据不同场景需求调整翻译的语言风格和表达方式。 - **上下文感知**: 支持通用、商务、技术、医疗、法律、市场营销、娱乐、教育、新闻等九种专业领域的上下文翻译，确保术语准确性和表达地道性。 - **格式保留**: 智能识别并保持原文的格式结构，包括换行、缩进、特殊符号等，确保翻译后的文本保持良好的可读性。  ## 支持的语言  我们支持超过100种语言的互译，详见下方参数列表。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target_lang** | **String** | 目标语言代码。请从[支持的语言列表](#enum-list)中选择一个语言代码。 | [required] |
**post_ai_translate_request** | [**PostAiTranslateRequest**](PostAiTranslateRequest.md) |  | [required] |

### Return type

[**models::PostAiTranslate200Response**](post_ai_translate_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_translate_stream

> String post_translate_stream(post_translate_stream_request)
流式翻译（中英互译）

想让翻译结果像打字机一样逐字显示出来？这个流式翻译接口能实现这种效果。  ## 功能概述 不同于传统翻译API一次性返回完整结果，这个接口会实时地、一个字一个字地把翻译内容推给你（就像ChatGPT回复消息那样），非常适合用在聊天应用、直播字幕等需要即时反馈的场景。  ## 它能做什么 - **中英互译**：支持中文和英文之间的双向翻译 - **自动识别**：不确定源语言？设置为 `auto` 让我们自动检测 - **逐字返回**：翻译结果会像打字机一样逐字流式返回，用户体验更流畅 - **音频朗读**：部分翻译结果会附带音频链接，方便朗读  ## 支持的语言 目前专注于中英互译，支持以下选项： - `中文`（简体/繁体） - `英文` - `auto`（自动检测）

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_translate_stream_request** | [**PostTranslateStreamRequest**](PostTranslateStreamRequest.md) |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_translate_text

> models::PostTranslateText200Response post_translate_text(to_lang, post_translate_text_request)
翻译

需要跨越语言的鸿沟进行交流？这个翻译接口是你可靠的'同声传译'。  ## 功能概述 你可以将一段源语言文本（我们能自动检测源语言）翻译成你指定的任何目标语言。无论是中译英、日译法，都不在话下。  ## 支持的语言 我们支持超过100种语言的互译，包括但不限于：中文（简体/繁体）、英语、日语、韩语、法语、德语、西班牙语、俄语、阿拉伯语等主流语言，以及各种小语种。详见下方参数列表。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**to_lang** | **String** | 目标语言代码。请从[支持的语言列表](#enum-list)中选择一个语言代码。 | [required] |
**post_translate_text_request** | [**PostTranslateTextRequest**](PostTranslateTextRequest.md) |  | [required] |

### Return type

[**models::PostTranslateText200Response**](post_translate_text_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

