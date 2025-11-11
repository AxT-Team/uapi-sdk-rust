# \RandomApi

All URIs are relative to *https://uapis.cn/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_answerbook_ask**](RandomApi.md#get_answerbook_ask) | **GET** /answerbook/ask | 获取答案之书的神秘答案 (GET)
[**get_random_image**](RandomApi.md#get_random_image) | **GET** /random/image | 随机二次元、风景、动漫图片壁纸
[**get_random_string**](RandomApi.md#get_random_string) | **GET** /random/string | 生成高度可定制的随机字符串
[**post_answerbook_ask**](RandomApi.md#post_answerbook_ask) | **POST** /answerbook/ask | 获取答案之书的神秘答案 (POST)



## get_answerbook_ask

> models::GetAnswerbookAsk200Response get_answerbook_ask(question)
获取答案之书的神秘答案 (GET)

想要获得人生问题的神秘答案吗？答案之书API提供了一个神奇8球风格的问答服务，你可以提问并获得随机的神秘答案。  ## 功能概述 通过向答案之书提问，你将获得一个充满智慧（或许）的随机答案。这个API支持通过查询参数或POST请求体两种方式提问。  ## 使用须知  > [!TIP] > **提问技巧** > - 提出明确的问题会获得更好的体验 > - 问题不能为空 > - 支持中文问题 > - 答案具有随机性，仅供娱乐参考

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**question** | **String** | 你想要提问的问题。问题不能为空。 | [required] |

### Return type

[**models::GetAnswerbookAsk200Response**](get_answerbook_ask_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_random_image

> std::path::PathBuf get_random_image(category, r#type)
随机二次元、风景、动漫图片壁纸

需要一张随机图片作为占位符或者背景吗？这个接口是你的不二之选。  ## 功能概述 这是一个非常简单的接口，它会从我们庞大的图库和精选外部图床中随机挑选一张图片，然后通过 302 重定向让你直接访问到它。这使得它可以非常方便地直接用在 HTML 的 `<img>` 标签中。  你可以通过 `/api/v1/random/image?category=acg&type=4k` 这样的请求获取由UapiPro服务器提供的图片，也可以通过 `/api/v1/random/image?category=ai_drawing` 获取由外部图床精选的图片。  如果你不提供任何 category 参数，程序会从所有图片（包括本地的和URL的）中随机抽取一张（**全局随机图片不包含ikun和AI绘画**）。  > [!TIP] > 如果你需要更精确地控制图片类型，请使用 `/image/random/{category}/{type}` 接口。  ### 支持的主类别与子类别 - **UapiPro服务器图片**   - **furry**（福瑞）     - z4k     - szs8k     - s4k     - 4k   - **bq**（表情包/趣图）     - youshou     - xiongmao     - waiguoren     - maomao     - ikun     - eciyuan   - **acg**（二次元动漫）     - pc     - mb - **外部图床精选图片**   - **ai_drawing**: AI绘画。   - **general_anime**: 动漫图。   - **landscape**: 风景图。   - **mobile_wallpaper**: 手机壁纸。   - **pc_wallpaper**: 电脑壁纸。 - **混合动漫**   - **anime**: 混合了UapiPro服务器的acg和外部图床的general_anime分类下的图片。  > [!NOTE] > 默认全局随机（未指定category参数）时，不会包含ikun和AI绘画（ai_drawing）类别的图片。 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category** | Option<**String**> | （可选）指定图片主类别。  **支持的主类别：** - `furry`（福瑞，UapiPro服务器） - `bq`（表情包/趣图，UapiPro服务器） - `acg`（二次元动漫，UapiPro服务器） - `ai_drawing`（AI绘画，外部图床） - `general_anime`（动漫图，外部图床） - `landscape`（风景图，外部图床） - `mobile_wallpaper`（手机壁纸，外部图床） - `pc_wallpaper`（电脑壁纸，外部图床） - `anime`（混合动漫，UapiPro服务器acg + 外部图床general_anime）  > [!TIP] > 如果不指定，将从所有图片中随机抽取（不包含 `ikun` 和 `ai_drawing`）。  |  |
**r#type** | Option<**String**> | （可选，仅UapiPro服务器图片支持）指定图片子类别。  - **furry**: `z4k`, `szs8k`, `s4k`, `4k` - **bq**: `youshou`, `xiongmao`, `waiguoren`, `maomao`, `ikun`, `eciyuan` - **acg**: `pc`, `mb`  > [!TIP] > 外部图床类别和 `anime` 混合类别不支持 `type` 参数。  |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/jpeg, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_random_string

> models::GetRandomString200Response get_random_string(length, r#type)
生成高度可定制的随机字符串

无论是需要生成一个安全的随机密码、一个唯一的Token，还是一个简单的随机ID，这个接口都能满足你。  ## 功能概述 你可以精确地控制生成字符串的长度和字符集类型，非常灵活。  ## 使用须知  > [!TIP] > **字符集类型 `type` 详解** > 你可以通过 `type` 参数精确控制生成的字符集： > - **`numeric`**: 纯数字 (0-9) > - **`lower`**: 纯小写字母 (a-z) > - **`upper`**: 纯大写字母 (A-Z) > - **`alpha`**: 大小写字母 (a-zA-Z) > - **`alphanumeric`** (默认): 数字和大小写字母 (0-9a-zA-Z) > - **`hex`**: 十六进制字符 (0-9a-f)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**length** | Option<**i32**> | 你希望生成的字符串的长度。有效范围是 1 到 1024。 |  |[default to 16]
**r#type** | Option<**String**> | 指定构成字符串的字符类型。 |  |[default to alphanumeric]

### Return type

[**models::GetRandomString200Response**](get_random_string_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_answerbook_ask

> models::PostAnswerbookAsk200Response post_answerbook_ask(post_answerbook_ask_request)
获取答案之书的神秘答案 (POST)

通过POST请求向答案之书提问并获得神秘答案。  ## 功能概述 与GET方式相同，但通过JSON请求体发送问题，适合在需要发送较长问题或希望避免URL编码问题的场景中使用。  ## 请求体格式 请求体必须是有效的JSON格式，包含question字段。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_answerbook_ask_request** | [**PostAnswerbookAskRequest**](PostAnswerbookAskRequest.md) | 包含问题的JSON对象 | [required] |

### Return type

[**models::PostAnswerbookAsk200Response**](post_answerbook_ask_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

