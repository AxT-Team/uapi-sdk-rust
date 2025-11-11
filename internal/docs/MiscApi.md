# \MiscApi

All URIs are relative to *https://uapis.cn/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_history_programmer**](MiscApi.md#get_history_programmer) | **GET** /history/programmer | 获取指定日期的程序员历史事件
[**get_history_programmer_today**](MiscApi.md#get_history_programmer_today) | **GET** /history/programmer/today | 获取今天的程序员历史事件
[**get_misc_hotboard**](MiscApi.md#get_misc_hotboard) | **GET** /misc/hotboard | 获取多平台实时热榜
[**get_misc_phoneinfo**](MiscApi.md#get_misc_phoneinfo) | **GET** /misc/phoneinfo | 查询手机号码归属地信息
[**get_misc_randomnumber**](MiscApi.md#get_misc_randomnumber) | **GET** /misc/randomnumber | 生成高度可定制的随机数
[**get_misc_timestamp**](MiscApi.md#get_misc_timestamp) | **GET** /misc/timestamp | 转换时间戳 (旧版，推荐使用/convert/unixtime)
[**get_misc_tracking_carriers**](MiscApi.md#get_misc_tracking_carriers) | **GET** /misc/tracking/carriers | 获取支持的快递公司列表
[**get_misc_tracking_detect**](MiscApi.md#get_misc_tracking_detect) | **GET** /misc/tracking/detect | 识别快递公司
[**get_misc_tracking_query**](MiscApi.md#get_misc_tracking_query) | **GET** /misc/tracking/query | 查询快递物流信息
[**get_misc_weather**](MiscApi.md#get_misc_weather) | **GET** /misc/weather | 查询实时天气信息
[**get_misc_worldtime**](MiscApi.md#get_misc_worldtime) | **GET** /misc/worldtime | 查询全球任意时区的时间



## get_history_programmer

> models::GetHistoryProgrammer200Response get_history_programmer(month, day)
获取指定日期的程序员历史事件

想查看程序员历史上某个特定日期发生的大事件？指定月份和日期，我们就能告诉你！  ## 功能概述 通过指定月份和日期，获取该日发生的程序员相关历史事件。同样使用AI智能筛选，确保事件的相关性和重要性。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**month** | **i32** | 月份，1-12之间的整数。 | [required] |
**day** | **i32** | 日期，1-31之间的整数。 | [required] |

### Return type

[**models::GetHistoryProgrammer200Response**](get_history_programmer_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_history_programmer_today

> models::GetHistoryProgrammerToday200Response get_history_programmer_today()
获取今天的程序员历史事件

想知道程序员历史上的今天发生了什么大事吗？这个接口告诉你答案！  ## 功能概述 我们使用AI智能筛选从海量历史事件中挑选出与程序员、计算机科学相关的重要事件。每个事件都经过重要性评分和相关性评分，确保内容质量。

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetHistoryProgrammerToday200Response**](get_history_programmer_today_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_misc_hotboard

> models::GetMiscHotboard200Response get_misc_hotboard(r#type)
获取多平台实时热榜

想快速跟上网络热点？这个接口让你一网打尽各大主流平台的实时热榜/热搜！  ## 功能概述 你只需要指定一个平台类型，就能获取到该平台当前的热榜数据列表。每个热榜条目都包含标题、热度值和原始链接。非常适合用于制作信息聚合类应用或看板。  ## 可选值 `type` 参数接受多种不同的值，每种值对应一个不同的热榜来源。以下是目前支持的所有值：  | 分类       | 支持的 type 值 | |------------|-----------------------------------------------------------------------------------------------------------------------------------| | 视频/社区  | bilibili（哔哩哔哩弹幕网）, acfun（A站弹幕视频网站）, weibo（新浪微博热搜）, zhihu（知乎热榜）, zhihu-daily（知乎日报热榜）, douyin（抖音热榜）, kuaishou（快手热榜）, douban-movie（豆瓣电影榜单）, douban-group（豆瓣小组话题）, tieba（百度贴吧热帖）, hupu（虎扑热帖）, miyoushe（米游社话题榜）, ngabbs（NGA游戏论坛热帖）, v2ex（V2EX技术社区热帖）, 52pojie（吾爱破解热帖）, hostloc（全球主机交流论坛）, coolapk（酷安热榜） | | 新闻/资讯  | baidu（百度热搜）, thepaper（澎湃新闻热榜）, toutiao（今日头条热榜）, qq-news（腾讯新闻热榜）, sina（新浪热搜）, sina-news（新浪新闻热榜）, netease-news（网易新闻热榜）, huxiu（虎嗅网热榜）, ifanr（爱范儿热榜） | | 技术/IT    | sspai（少数派热榜）, ithome（IT之家热榜）, ithome-xijiayi（IT之家·喜加一栏目）, juejin（掘金社区热榜）, jianshu（简书热榜）, guokr（果壳热榜）, 36kr（36氪热榜）, 51cto（51CTO热榜）, csdn（CSDN博客热榜）, nodeseek（NodeSeek 技术社区）, hellogithub（HelloGitHub 项目推荐） | | 游戏       | lol（英雄联盟热帖）, genshin（原神热榜）, honkai（崩坏3热榜）, starrail（星穹铁道热榜） | | 其他       | weread（微信读书热门书籍）, weatheralarm（天气预警信息）, earthquake（地震速报）, history（历史上的今天） | 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | 你想要查询的热榜平台。支持多种主流平台类型，详见下方[可选值](#可选值)表格。 | [required] |

### Return type

[**models::GetMiscHotboard200Response**](get_misc_hotboard_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_misc_phoneinfo

> models::GetMiscPhoneinfo200Response get_misc_phoneinfo(phone)
查询手机号码归属地信息

想知道一个手机号码来自哪里？是移动、联通还是电信？这个接口可以告诉你答案。  ## 功能概述 提供一个国内的手机号码，我们会查询并返回它的归属地（省份和城市）以及所属的运营商信息。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**phone** | **String** | 需要查询的11位中国大陆手机号码。 | [required] |

### Return type

[**models::GetMiscPhoneinfo200Response**](get_misc_phoneinfo_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_misc_randomnumber

> models::GetMiscRandomnumber200Response get_misc_randomnumber(min, max, count, allow_repeat, allow_decimal, decimal_places)
生成高度可定制的随机数

需要一个简单的随机数，还是需要一串不重复的、带小数的随机数？这个接口都能满足你！  ## 功能概述 这是一个强大的随机数生成器。你可以指定生成的范围（最大/最小值）、数量、是否允许重复、以及是否生成小数（并指定小数位数）。  ## 流程图 ```mermaid graph TD     A[开始] --> B{参数校验};     B --> |通过| C{是否允许小数?};     C --> |是| D[生成随机小数];     C --> |否| E[生成随机整数];     D --> F{是否允许重复?};     E --> F;     F --> |是| G[直接生成指定数量];     F --> |否| H[生成不重复的数字];     G --> I[返回结果];     H --> I;     B --> |失败| J[返回 400 错误]; ``` ## 使用须知 > [!WARNING] > **不重复生成的逻辑限制** > 当设置 `allow_repeat=false` 时，请确保取值范围 `(max - min + 1)` 大于或等于你请求的数量 `count`。否则，系统将无法生成足够的不重复数字，请求会失败并返回 400 错误。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**min** | Option<**i32**> | 生成随机数的最小值（包含）。 |  |[default to 1]
**max** | Option<**i32**> | 生成随机数的最大值（包含）。 |  |[default to 100]
**count** | Option<**i32**> | 需要生成的随机数的数量。 |  |[default to 1]
**allow_repeat** | Option<**bool**> | 是否允许生成的多个数字中出现重复值。 |  |[default to false]
**allow_decimal** | Option<**bool**> | 是否生成小（浮点）数。如果为 false，则只生成整数。 |  |[default to false]
**decimal_places** | Option<**i32**> | 如果 `allow_decimal=true`，这里可以指定小数的位数。 |  |[default to 2]

### Return type

[**models::GetMiscRandomnumber200Response**](get_misc_randomnumber_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_misc_timestamp

> models::GetMiscTimestamp200Response get_misc_timestamp(ts)
转换时间戳 (旧版，推荐使用/convert/unixtime)

这是一个用于将Unix时间戳转换为人类可读日期时间的旧版接口。  ## 功能概述 输入一个秒级或毫秒级的时间戳，返回其对应的本地时间和UTC时间。  > [!WARNING] > **接口已过时**：这个接口已被新的 `/convert/unixtime` 取代。新接口功能更强大，支持双向转换。我们建议你迁移到新接口。  [👉 前往新版接口文档](/docs/api-reference/get-convert-unixtime)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ts** | **String** | 需要转换的Unix时间戳，支持10位（秒）或13位（毫秒）。 | [required] |

### Return type

[**models::GetMiscTimestamp200Response**](get_misc_timestamp_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_misc_tracking_carriers

> models::GetMiscTrackingCarriers200Response get_misc_tracking_carriers()
获取支持的快递公司列表

不确定系统支持哪些快递公司？这个接口返回完整的支持列表。  > [!VIP] > 本API目前处于**限时免费**阶段，我们鼓励开发者集成和测试。未来，它将转为付费API，为用户提供更稳定和强大的服务。  ## 功能概述 获取系统当前支持的所有快递公司列表，包括每家公司的标准编码（code）和中文名称（name）。  ## 使用建议 - **推荐缓存**：这个列表基本不会频繁变动，建议在应用启动时调用一次并缓存到本地 - **应用场景**：适合用于构建快递公司选择器、下拉菜单等UI组件 - **缓存时长**：建议缓存24小时或更久

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetMiscTrackingCarriers200Response**](get_misc_tracking_carriers_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_misc_tracking_detect

> models::GetMiscTrackingDetect200Response get_misc_tracking_detect(tracking_number)
识别快递公司

不确定手里的快递单号属于哪家快递公司？这个接口专门做识别，不查物流。  > [!VIP] > 本API目前处于**限时免费**阶段，我们鼓励开发者集成和测试。未来，它将转为付费API，为用户提供更稳定和强大的服务。  ## 功能概述 输入快递单号，系统会根据单号规则快速识别出最可能的快递公司。如果存在多个可能的匹配结果，还会在 `alternatives` 字段中返回备选项，供你参考选择。  ## 使用须知 - **识别速度快**：只做规则匹配，不查询物流信息，响应速度通常在100ms内 - **准确率高**：基于各快递公司的单号规则进行智能识别，准确率超过95% - **备选方案**：当单号规则可能匹配多家快递公司时，会提供所有可能的选项

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tracking_number** | **String** | 需要识别的快递单号。 | [required] |

### Return type

[**models::GetMiscTrackingDetect200Response**](get_misc_tracking_detect_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_misc_tracking_query

> models::GetMiscTrackingQuery200Response get_misc_tracking_query(tracking_number, carrier_code)
查询快递物流信息

买了东西想知道快递到哪儿了？这个接口帮你实时追踪物流状态。  > [!VIP] > 本API目前处于**限时免费**阶段，我们鼓励开发者集成和测试。未来，它将转为付费API，为用户提供更稳定和强大的服务。  ## 功能概述 提供一个快递单号，系统会自动识别快递公司并返回完整的物流轨迹信息。支持中通、圆通、韵达、申通、极兔、顺丰、京东、EMS、德邦等60+国内外主流快递公司。  ## 使用须知 - **自动识别**：不知道是哪家快递？系统会根据单号规则自动识别快递公司（推荐使用） - **手动指定**：如果已知快递公司，可以传递 `carrier_code` 参数，查询速度会更快 - **查询时效**：物流信息实时查询，响应时间通常在1-2秒内

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tracking_number** | **String** | 快递单号，通常是一串10-20位的数字或字母数字组合。 | [required] |
**carrier_code** | Option<**String**> | 快递公司编码（可选）。不填写时系统会自动识别，填写后可加快查询速度。 |  |

### Return type

[**models::GetMiscTrackingQuery200Response**](get_misc_tracking_query_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_misc_weather

> models::GetMiscWeather200Response get_misc_weather(city, adcode)
查询实时天气信息

出门前，查一下天气总是个好习惯。这个接口为你提供精准、实时的天气数据。  ## 功能概述 你可以通过城市名称或高德地图的Adcode来查询指定地区的实时天气状况，包括天气现象、温度、湿度、风向和风力等。  ## 使用须知 - **参数优先级**：当你同时提供了 `city` (城市名) 和 `adcode` (城市编码) 两个参数时，系统会 **优先使用 `adcode`** 进行查询，因为它更精确。 - **查询范围**：为了保证查询的准确性，我们的服务仅支持标准的“省”、“市”、“区/县”级别的行政区划名称查询，不保证能查询到乡镇或具体地点。  ## 错误处理指南 - **410 Gone**: 这个特殊的错误码意味着你查询的地区无效或不受我们支持。比如你输入了“火星”，或者某个我们无法识别的村庄名称。这个状态码告诉你，这个“资源”是永久性地不可用了。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**city** | Option<**String**> | 标准的城市名称，如 '北京', '上海市', '福田区'。请使用官方的省、市、区县行政区划名称。 |  |
**adcode** | Option<**String**> | 高德地图的6位数字城市编码。例如，北京市的Adcode是 '110000'。使用Adcode查询更准确、更快速。 |  |

### Return type

[**models::GetMiscWeather200Response**](get_misc_weather_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_misc_worldtime

> models::GetMiscWorldtime200Response get_misc_worldtime(city)
查询全球任意时区的时间

需要和国外的朋友开会，想知道他那边现在几点？用这个接口一查便知。  ## 功能概述 根据标准的时区名称（例如 'Asia/Shanghai' 或 'Europe/London'），获取该时区的当前准确时间、UTC偏移量、星期等信息。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**city** | **String** | 你需要查询的城市或地区，请使用标准的 IANA 时区数据库名称，例如 'Shanghai', 'Asia/Tokyo', 'America/New_York'。 | [required] |

### Return type

[**models::GetMiscWorldtime200Response**](get_misc_worldtime_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

