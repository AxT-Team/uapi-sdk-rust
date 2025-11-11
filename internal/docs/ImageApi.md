# \ImageApi

All URIs are relative to *https://uapis.cn/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_avatar_gravatar**](ImageApi.md#get_avatar_gravatar) | **GET** /avatar/gravatar | 获取Gravatar头像
[**get_image_bing_daily**](ImageApi.md#get_image_bing_daily) | **GET** /image/bing-daily | 获取必应每日壁纸
[**get_image_motou**](ImageApi.md#get_image_motou) | **GET** /image/motou | 生成摸摸头GIF (QQ号方式)
[**get_image_qrcode**](ImageApi.md#get_image_qrcode) | **GET** /image/qrcode | 动态生成二维码
[**get_image_tobase64**](ImageApi.md#get_image_tobase64) | **GET** /image/tobase64 | 将在线图片转换为Base64
[**post_image_compress**](ImageApi.md#post_image_compress) | **POST** /image/compress | 无损压缩图片
[**post_image_frombase64**](ImageApi.md#post_image_frombase64) | **POST** /image/frombase64 | 通过Base64编码上传图片
[**post_image_motou**](ImageApi.md#post_image_motou) | **POST** /image/motou | 生成摸摸头GIF (图片上传或URL方式)
[**post_image_speechless**](ImageApi.md#post_image_speechless) | **POST** /image/speechless | 生成你们怎么不说话了表情包
[**post_image_svg**](ImageApi.md#post_image_svg) | **POST** /image/svg | SVG转图片



## get_avatar_gravatar

> std::path::PathBuf get_avatar_gravatar(email, hash, s, d, r)
获取Gravatar头像

提供一个超高速、高可用的Gravatar头像代理服务。内置了强大的ETag条件缓存，确保用户在更新Gravatar头像后能几乎立刻看到变化，同时最大化地利用缓存。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | Option<**String**> | 用户的 Email 地址。如果未提供 `hash` 参数，则此参数为必需。 |  |
**hash** | Option<**String**> | 用户 Email 地址的小写 MD5 哈希值。如果提供此参数，将忽略 `email` 参数。 |  |
**s** | Option<**i32**> | 头像的尺寸，单位为像素。有效范围是 1 到 2048。 |  |[default to 80]
**d** | Option<**String**> | 当用户没有自己的 Gravatar 头像时，显示的默认头像类型。可选值包括 `mp`, `identicon`, `monsterid`, `wavatar`, `retro`, `robohash`, `blank`, `404`。 |  |[default to mp]
**r** | Option<**String**> | 头像分级。可选值：`g`, `pg`, `r`, `x`。 |  |[default to g]

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_image_bing_daily

> std::path::PathBuf get_image_bing_daily()
获取必应每日壁纸

每天都想换张新壁纸？让必应的美图点亮你的一天吧！  ## 功能概述 这个接口会获取 Bing 搜索引擎当天全球同步的每日壁纸，并直接以图片形式返回。你可以用它来做应用的启动页、网站背景，或者任何需要每日更新精美图片的地方。  ## 使用须知  > [!NOTE] > **响应格式是图片** > 请注意，此接口成功时直接返回图片二进制数据（通常为 `image/jpeg`），而非 JSON 格式。请确保客户端能够正确处理。  我们内置了备用方案：如果从必应官方获取图片失败，系统会尝试返回一张预存的高质量风景图，以保证服务的稳定性。

### Parameters

This endpoint does not need any parameter.

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_image_motou

> std::path::PathBuf get_image_motou(qq, bg_color)
生成摸摸头GIF (QQ号方式)

想在线rua一下好友的头像吗？这个趣味接口可以满足你。  ## 功能概述 此接口通过GET方法，专门用于通过QQ号生成摸摸头GIF。你只需要提供一个QQ号码，我们就会自动获取其公开头像，并制作成一个可爱的动图。  ## 使用须知 - **响应格式**：接口成功时直接返回 `image/gif` 格式的二进制数据。 - **背景颜色**：你可以通过 `bg_color` 参数来控制GIF的背景。使用 `transparent` 选项可以让它更好地融入各种聊天背景中。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**qq** | **String** | 你想要摸头的对象的QQ号码。 | [required] |
**bg_color** | Option<**String**> | GIF的背景颜色。留空则由后端服务决定默认值。 |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/gif, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_image_qrcode

> std::path::PathBuf get_image_qrcode(text, size, format)
动态生成二维码

无论是网址、文本还是联系方式，通通可以变成一个二维码！这是一个非常灵活的二维码生成工具。  ## 功能概述 你提供一段文本内容，我们为你生成对应的二维码图片。你可以自定义尺寸，并选择不同的返回格式以适应不同场景。  ## 使用须知  > [!IMPORTANT] > **关键参数 `format`** > 此参数决定了成功响应的内容类型和结构，请务必根据你的需求选择并正确处理响应： > - **`image`** (默认): 直接返回 `image/png` 格式的图片二进制数据，适合在 `<img>` 标签中直接使用。 > - **`json`**: 返回一个包含 Base64 Data URI 的 JSON 对象，适合需要在前端直接嵌入CSS或HTML的场景。 > - **`json_url`**: 返回一个包含图片临时URL的JSON对象，适合需要图片链接的场景。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**text** | **String** | 你希望编码到二维码中的任何文本内容，比如一个URL、一段话或者一个JSON字符串。 | [required] |
**size** | Option<**i32**> | 二维码图片的边长（正方形），单位是像素。有效范围是 256 到 1024 之间。 |  |[default to 256]
**format** | Option<**String**> | 指定响应内容的格式。可选值为 `image`, `json`, `json_url`。 |  |[default to image]

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/png, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_image_tobase64

> models::GetImageTobase64200Response get_image_tobase64(url)
将在线图片转换为Base64

看到一张网上的图片，想把它转换成 Base64 编码以便嵌入到你的 HTML 或 CSS 中？用这个接口就对了。  ## 功能概述 你提供一个公开可访问的图片 URL，我们帮你把它下载下来，并转换成包含 MIME 类型的 Base64 Data URI 字符串返回给你。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url** | **String** | 需要转换为Base64的、可公开访问的图片URL地址。 | [required] |

### Return type

[**models::GetImageTobase64200Response**](get_image_tobase64_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_image_compress

> std::path::PathBuf post_image_compress(file, level, format)
无损压缩图片

还在为图片体积和加载速度发愁吗？体验一下我们强大的**无损压缩服务**，它能在几乎不牺牲任何肉眼可感知的画质的前提下，将图片体积压缩到极致。  ## 功能概述 你只需要上传一张常见的图片（如 PNG, JPG），选择一个压缩等级，就能获得一个体积小到惊人的压缩文件。这对于需要大量展示高清图片的网站、App 或小程序来说，是优化用户体验、节省带宽和存储成本的利器。  ## 使用须知 > [!TIP] > 为了给您最好的压缩效果，我们的算法需要进行复杂计算，处理时间可能会稍长一些，请耐心等待。  > [!WARNING] > **服务排队提醒** > 这是一个计算密集型服务。在高并发时，您的请求可能会被排队等待处理。如果您需要将其集成到对延迟敏感的生产服务中，请注意这一点。  ### 请求与响应格式 - 请求必须使用 `multipart/form-data` 格式上传文件。 - 成功响应将直接返回压缩后的文件二进制流 (`application/octet-stream`)，并附带 `Content-Disposition` 头，建议客户端根据此头信息保存文件。  ## 参数详解 ### `level` (压缩等级) 这是一个从 `1` 到 `5` 的整数，它决定了压缩的强度和策略，数字越小，压缩率越高。所有等级都经过精心调校，以在最大化压缩率的同时保证出色的视觉质量。 - `1`: **极限压缩** (推荐，体积最小，画质优异) - `2`: **高效压缩** - `3`: **智能均衡** (默认选项) - `4`: **画质优先** - `5`: **专业保真** (压缩率稍低，保留最多图像信息)  ## 错误处理指南 - **400 Bad Request**: 通常因为没有上传文件，或者 `level` 参数不在 1-5 的范围内。 - **500 Internal Server Error**: 如果在压缩过程中服务器发生内部错误，会返回此状态码。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | **std::path::PathBuf** | 支持PNG, JPG, JPEG等常见图片格式。文件大小不超过15MB。 | [required] |
**level** | Option<**i32**> | 压缩强度 (1-5)，默认为 3。数字越小，压缩率越高。 |  |[default to 3]
**format** | Option<**String**> | 输出图片格式，可以是 'png' 或 'jpeg'。 |  |[default to png]

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: image/*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_image_frombase64

> models::PostImageFrombase64200Response post_image_frombase64(post_image_frombase64_request)
通过Base64编码上传图片

当你需要在前端处理完图片（比如裁剪、加滤镜后），不通过传统表单，而是直接上传图片的场景，这个接口就派上用场了。  ## 功能概述 你只需要将图片的 Base64 编码字符串发送过来，我们就会把它解码、保存为图片文件，并返回一个可供访问的公开 URL。  ## 使用须知  > [!IMPORTANT] > **关于 `imageData` 格式** > 你发送的 `imageData` 字符串必须是完整的 Base64 Data URI 格式，它需要包含 MIME 类型信息，例如 `data:image/png;base64,iVBORw0KGgo...`。缺少 `data:image/...;base64,` 前缀将导致解码失败。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_image_frombase64_request** | [**PostImageFrombase64Request**](PostImageFrombase64Request.md) | 一个JSON对象，包含 `imageData` 字段，其值为你想要上传图片的完整Base64 Data URI。 | [required] |

### Return type

[**models::PostImageFrombase64200Response**](post_image_frombase64_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_image_motou

> std::path::PathBuf post_image_motou(image_url, file, bg_color)
生成摸摸头GIF (图片上传或URL方式)

除了使用QQ头像，你还可以通过上传自己的图片或提供图片URL来制作独一无二的摸摸头GIF。  ## 功能概述 此接口通过POST方法，支持两种方式生成GIF： 1.  **图片URL**：在表单中提供 `image_url` 字段。 2.  **上传图片**：在表单中上传 `file` 文件。  ## 使用须知 - **响应格式**：接口成功时直接返回 `image/gif` 格式的二进制数据。 - **参数优先级**：如果同时提供了 `image_url` 和上传的 `file` 文件，系统将 **优先使用 `image_url`**。 - **背景颜色**：同样支持 `bg_color` 表单字段来控制GIF背景。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**image_url** | Option<**String**> | 图片的URL地址。如果提供此项，将优先使用该URL的图片。 |  |
**file** | Option<**std::path::PathBuf**> | 上传的图片文件。支持JPG、PNG、GIF等常见格式。 |  |
**bg_color** | Option<**String**> | GIF的背景颜色。可选值为 'white', 'black', 'transparent'。 |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: image/gif, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_image_speechless

> std::path::PathBuf post_image_speechless(post_image_speechless_request)
生成你们怎么不说话了表情包

你们怎么不说话了？是不是都在偷偷玩Uapi，求求你们不要玩Uapi了  ## 效果展示 ![示例](https://uapis.cn/static/uploads/33580466897f1e5815296f235b582815.png)  ## 使用须知 - **响应格式**：接口成功时直接返回 `image/jpeg` 格式的二进制数据。 - **文字内容**：至少需要提供 `top_text`（上方文字）或 `bottom_text`（下方文字）之一。 - **梗图逻辑**：上方描述某个行为，下方通常以「们」开头表示劝阻，形成戏谑的对比效果。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_image_speechless_request** | [**PostImageSpeechlessRequest**](PostImageSpeechlessRequest.md) | 包含表情包文字内容的JSON对象。至少需要提供上方或下方文字之一。 | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: image/png, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_image_svg

> std::path::PathBuf post_image_svg(format, width, height, quality, file)
SVG转图片

需要将灵活的 SVG 矢量图形转换为常见的光栅图像格式吗？这个接口可以帮你轻松实现。  ## 功能概述 上传一个 SVG 文件，并指定目标格式（如 PNG、JPEG 等），接口将返回转换后的图像。你还可以调整输出图像的尺寸和（对于JPEG）压缩质量，以满足不同场景的需求。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | 输出图像的目标格式。支持的值：`png`, `jpeg`, `jpg`, `gif`, `tiff`, `bmp`。 |  |[default to png]
**width** | Option<**i32**> | 输出图像的宽度（像素）。如果省略，将根据 `height` 保持宽高比，或者使用 SVG 的原始宽度。 |  |
**height** | Option<**i32**> | 输出图像的高度（像素）。如果省略，将根据 `width` 保持宽高比，或者使用 SVG 的原始高度。 |  |
**quality** | Option<**i32**> | JPEG 图像的压缩质量（1-100）。仅当 `format` 为 `jpeg` 或 `jpg` 时有效。 |  |[default to 90]
**file** | Option<**std::path::PathBuf**> | 支持SVG文件 |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: image/*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

