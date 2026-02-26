# \MiscApi

All URIs are relative to *https://uapis.cn/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_history_programmer**](MiscApi.md#get_history_programmer) | **GET** /history/programmer | 程序员历史事件
[**get_history_programmer_today**](MiscApi.md#get_history_programmer_today) | **GET** /history/programmer/today | 程序员历史上的今天
[**get_misc_district**](MiscApi.md#get_misc_district) | **GET** /misc/district | Adcode 国内外行政区域查询
[**get_misc_holiday_calendar**](MiscApi.md#get_misc_holiday_calendar) | **GET** /misc/holiday-calendar | 查询节假日与万年历
[**get_misc_hotboard**](MiscApi.md#get_misc_hotboard) | **GET** /misc/hotboard | 查询热榜
[**get_misc_lunartime**](MiscApi.md#get_misc_lunartime) | **GET** /misc/lunartime | 查询农历时间
[**get_misc_phoneinfo**](MiscApi.md#get_misc_phoneinfo) | **GET** /misc/phoneinfo | 查询手机归属地
[**get_misc_randomnumber**](MiscApi.md#get_misc_randomnumber) | **GET** /misc/randomnumber | 随机数生成
[**get_misc_timestamp**](MiscApi.md#get_misc_timestamp) | **GET** /misc/timestamp | 转换时间戳 (旧版，推荐使用/convert/unixtime)
[**get_misc_tracking_carriers**](MiscApi.md#get_misc_tracking_carriers) | **GET** /misc/tracking/carriers | 获取支持的快递公司列表
[**get_misc_tracking_detect**](MiscApi.md#get_misc_tracking_detect) | **GET** /misc/tracking/detect | 识别快递公司
[**get_misc_tracking_query**](MiscApi.md#get_misc_tracking_query) | **GET** /misc/tracking/query | 查询快递物流信息
[**get_misc_weather**](MiscApi.md#get_misc_weather) | **GET** /misc/weather | 查询天气
[**get_misc_worldtime**](MiscApi.md#get_misc_worldtime) | **GET** /misc/worldtime | 查询世界时间
[**post_misc_date_diff**](MiscApi.md#post_misc_date_diff) | **POST** /misc/date-diff | 计算两个日期之间的时间差值



## get_history_programmer

> models::GetHistoryProgrammer200Response get_history_programmer(month, day)
程序员历史事件

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
程序员历史上的今天

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


## get_misc_district

> models::GetMiscDistrict200Response get_misc_district(keywords, adcode, lat, lng, level, country, limit)
Adcode 国内外行政区域查询

一个接口，覆盖全球 243 个国家、中国省/市/区/街道四级行政区划，支持关键词搜索、行政编码查询、坐标反查三种查询模式（必须至少传入一种查询参数）。  ## 功能概述 根据用户输入的搜索条件快速查找行政区域信息。例如：中国 > 山东省 > 济南市 > 历下区 > 舜华路街道。  无需注册、无需密钥，直接调用即可获取结构化的行政区域数据。支持三种查询方式： - 传 `adcode`，按行政编码精确查询，同时返回下级区划列表 - 传 `lat` + `lng`，坐标反查附近地点 - 传 `keywords`，按关键词搜索，支持中英文  ## 中国与国际数据差异 中国数据包含 `adcode`、`citycode` 等字段，支持省/市/区/街道四级逐级查询；国际城市数据不含这些字段，但额外提供 `population`（人口）和 `timezone`（时区）。  > [!NOTE] > 部分城市（如东莞、文昌）没有区县层级，市级下方直接显示街道。街道级别的 `adcode` 返回的是所属区县的 `adcode`。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**keywords** | Option<**String**> | 关键词搜索（城市名、区县名，支持中英文）。 |  |
**adcode** | Option<**String**> | 中国行政区划代码精确查询（如 `110000`），同时返回下级行政区。 |  |
**lat** | Option<**f64**> | 纬度，与 `lng` 配合使用，坐标反查附近地点。 |  |
**lng** | Option<**f64**> | 经度，与 `lat` 配合使用。 |  |
**level** | Option<**String**> | 过滤行政级别。 |  |
**country** | Option<**String**> | 过滤国家代码（ISO 3166-1 alpha-2），如 `CN`、`JP`、`US`、`GB`。 |  |
**limit** | Option<**i32**> | 返回数量上限，默认 `20`，最大 `100`。 |  |[default to 20]

### Return type

[**models::GetMiscDistrict200Response**](get_misc_district_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_misc_holiday_calendar

> models::GetMiscHolidayCalendar200Response get_misc_holiday_calendar(date, month, year, timezone, holiday_type, include_nearby, nearby_limit)
查询节假日与万年历

查询指定日期、月份或年份的万年历与节假日信息。  ## 功能概述 这个接口支持三种查询方式：按天（`date`）、按月（`month`）和按年（`year`）。调用时三者选一个传入即可。  如果你只关心某一类事件，可以通过 `holiday_type` 进行筛选，例如只看法定休假/调休、公历节日、农历节日或节气。  在 `date` 模式下，传 `include_nearby=true` 可以额外返回该日期前后最近的节日；返回数量由 `nearby_limit` 控制，默认 7，最大 30。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date** | Option<**String**> | 按天查询时填写这个参数，例如查某一天。格式：`YYYY-MM-DD`。和 `month`、`year` 三选一。 |  |
**month** | Option<**String**> | 按月查询时填写这个参数，例如查某个月。格式：`YYYY-MM`。和 `date`、`year` 三选一。 |  |
**year** | Option<**String**> | 按年查询时填写这个参数，例如查某一年。格式：`YYYY`。和 `date`、`month` 三选一。 |  |
**timezone** | Option<**String**> | 时区名称，默认 Asia/Shanghai。 |  |[default to Asia/Shanghai]
**holiday_type** | Option<**String**> | 节日筛选类型，默认 all。 |  |[default to all]
**include_nearby** | Option<**bool**> | 是否返回前后最近节日，仅 date 模式生效，默认 false。month/year 模式会忽略此参数。 |  |[default to false]
**nearby_limit** | Option<**i32**> | 返回最近节日数量限制，默认 7，最大 30。仅 date 模式 + include_nearby=true 生效。 |  |[default to 7]

### Return type

[**models::GetMiscHolidayCalendar200Response**](get_misc_holiday_calendar_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_misc_hotboard

> models::GetMiscHotboard200Response get_misc_hotboard(r#type, time, keyword, time_start, time_end, limit, sources)
查询热榜

想快速跟上网络热点？这个接口让你一网打尽各大主流平台的实时热榜/热搜！  ## 功能概述 你只需要指定一个平台类型，就能获取到该平台当前的热榜数据列表。每个热榜条目都包含标题、热度值和原始链接。非常适合用于制作信息聚合类应用或看板。  ## 三种使用模式  ### 默认模式 只传 `type` 参数，返回该平台当前的实时热榜。  ### 时光机模式 传 `type` + `time` 参数，返回最接近指定时间的热榜快照。如果不可用或无数据，会返回空。  ### 搜索模式 传 `type` + `keyword` + `time_start` + `time_end` 参数，在指定时间范围内搜索包含关键词的热榜条目。可选传 `limit` 限制返回数量。  ### 数据源列表 传 `sources=true`，返回所有支持历史数据的平台列表。  ## 可选值 `type` 参数接受多种不同的值，每种值对应一个不同的热榜来源。以下是目前支持的所有值：  | 分类       | 支持的 type 值 | |------------|-----------------------------------------------------------------------------------------------------------------------------------| | 视频/社区  | bilibili（哔哩哔哩弹幕网）, acfun（A站弹幕视频网站）, weibo（新浪微博热搜）, zhihu（知乎热榜）, zhihu-daily（知乎日报热榜）, douyin（抖音热榜）, kuaishou（快手热榜）, douban-movie（豆瓣电影榜单）, douban-group（豆瓣小组话题）, tieba（百度贴吧热帖）, hupu（虎扑热帖）, ngabbs（NGA游戏论坛热帖）, v2ex（V2EX技术社区热帖）, 52pojie（吾爱破解热帖）, hostloc（全球主机交流论坛）, coolapk（酷安热榜） | | 新闻/资讯  | baidu（百度热搜）, thepaper（澎湃新闻热榜）, toutiao（今日头条热榜）, qq-news（腾讯新闻热榜）, sina（新浪热搜）, sina-news（新浪新闻热榜）, netease-news（网易新闻热榜）, huxiu（虎嗅网热榜）, ifanr（爱范儿热榜） | | 技术/IT    | sspai（少数派热榜）, ithome（IT之家热榜）, ithome-xijiayi（IT之家·喜加一栏目）, juejin（掘金社区热榜）, jianshu（简书热榜）, guokr（果壳热榜）, 36kr（36氪热榜）, 51cto（51CTO热榜）, csdn（CSDN博客热榜）, nodeseek（NodeSeek 技术社区）, hellogithub（HelloGitHub 项目推荐） | | 游戏       | lol（英雄联盟热帖）, genshin（原神热榜）, honkai（崩坏3热榜）, starrail（星穹铁道热榜） | | 音乐       | netease-music（网易云音乐热歌榜）, qq-music（QQ音乐热歌榜） | | 其他       | weread（微信读书热门书籍）, weatheralarm（天气预警信息）, earthquake（地震速报）, history（历史上的今天） | 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | 你想要查询的热榜平台。支持多种主流平台类型，详见下方[可选值](#可选值)表格。 | [required] |
**time** | Option<**i64**> | 时光机模式：毫秒时间戳，返回最接近该时间的热榜快照。不传则返回当前实时热榜。 |  |
**keyword** | Option<**String**> | 搜索模式：搜索关键词，在历史热榜中搜索包含该关键词的条目。需配合 time_start 和 time_end 使用。 |  |
**time_start** | Option<**i64**> | 搜索模式必填：搜索起始时间戳（毫秒）。 |  |
**time_end** | Option<**i64**> | 搜索模式必填：搜索结束时间戳（毫秒）。 |  |
**limit** | Option<**i32**> | 搜索模式下最大返回条数，默认 50，最大 200。 |  |[default to 50]
**sources** | Option<**bool**> | 设为 true 时列出所有可用的历史数据源，忽略其他参数。 |  |

### Return type

[**models::GetMiscHotboard200Response**](get_misc_hotboard_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_misc_lunartime

> models::GetMiscLunartime200Response get_misc_lunartime(ts, timezone)
查询农历时间

需要在指定时区下查看某个时间点的农历信息？这个接口可以直接返回完整结果。  ## 功能概述 支持传入 Unix 时间戳（秒或毫秒）和 IANA 时区名，返回公历时间、星期、农历年月日、干支、生肖、节气与节日信息。不传 `ts` 时默认使用当前时间，不传 `timezone` 时默认 `Asia/Shanghai`。  ## 时区说明 - 支持标准 IANA 时区，例如 `Asia/Shanghai`、`Asia/Tokyo` - 也支持别名：`Shanghai`、`Beijing` - 时区非法时返回 400 并提示 `invalid timezone: xxx`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ts** | Option<**String**> | Unix 时间戳，支持 10 位秒级或 13 位毫秒级。不传则默认当前时间。 |  |
**timezone** | Option<**String**> | 时区名称。支持 IANA 时区（如 Asia/Shanghai）和别名（Shanghai、Beijing）。默认 Asia/Shanghai。 |  |

### Return type

[**models::GetMiscLunartime200Response**](get_misc_lunartime_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_misc_phoneinfo

> models::GetMiscPhoneinfo200Response get_misc_phoneinfo(phone)
查询手机归属地

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
随机数生成

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

这是一个用于将Unix时间戳转换为人类可读日期时间的旧版接口。  ## 功能概述 输入一个秒级或毫秒级的时间戳，返回其对应的本地时间和UTC时间。  > [!WARNING] > **接口已过时**：这个接口已被新的 `/convert/unixtime` 取代。新接口功能更强大，支持双向转换。我们建议你迁移到新接口。  [➡️ 前往新版接口文档](/docs/api-reference/get-convert-unixtime)

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

> models::GetMiscTrackingQuery200Response get_misc_tracking_query(tracking_number, carrier_code, phone)
查询快递物流信息

买了东西想知道快递到哪儿了？这个接口帮你实时追踪物流状态。  > [!VIP] > 本API目前处于**限时免费**阶段，我们鼓励开发者集成和测试。未来，它将转为付费API，为用户提供更稳定和强大的服务。  ## 功能概述 提供一个快递单号，系统会自动识别快递公司并返回完整的物流轨迹信息。支持中通、圆通、韵达、申通、极兔、顺丰、京东、EMS、德邦等60+国内外主流快递公司。  ## 使用须知 - **自动识别**：不知道是哪家快递？系统会根据单号规则自动识别快递公司（推荐使用） - **手动指定**：如果已知快递公司，可以传递 `carrier_code` 参数，查询速度会更快 - **手机尾号验证**：部分快递公司需要验证收件人手机尾号才能查询详细物流，如果返回「暂无物流信息」，建议尝试传入 `phone` 参数 - **查询时效**：物流信息实时查询，响应时间通常在1-2秒内

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tracking_number** | **String** | 快递单号，通常是一串10-20位的数字或字母数字组合。 | [required] |
**carrier_code** | Option<**String**> | 快递公司编码（可选）。不填写时系统会自动识别，填写后可加快查询速度。 |  |
**phone** | Option<**String**> | 收件人手机尾号，4位数字（可选）。部分快递公司需要验证手机尾号才能查询详细物流信息。 |  |

### Return type

[**models::GetMiscTrackingQuery200Response**](get_misc_tracking_query_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_misc_weather

> models::GetMiscWeather200Response get_misc_weather(city, adcode, extended, forecast, hourly, minutely, indices, lang)
查询天气

出门前，查一下天气总是个好习惯。这个接口为你提供精准、实时的天气数据，支持国内和国际城市。  ## 功能概述 这个接口支持三种查询方式： - 可以传 `adcode`，按行政区编码查询（优先级最高） - 可以传 `city`，按城市名称查询，支持中文（`北京`）和英文（`Tokyo`） - 两个都不传时，按客户端 IP 自动定位查询  支持 `lang` 参数，可选 `zh`（默认）和 `en`，城市名翻译覆盖 7000+ 城市。  ## 可选功能模块 - `extended=true`：扩展气象字段（体感温度、能见度、气压、紫外线、空气质量及污染物分项数据） - `forecast=true`：多天预报（最多7天，含日出日落、风速等详细数据） - `hourly=true`：逐小时预报（24小时） - `minutely=true`：分钟级降水预报（仅国内城市） - `indices=true`：18项生活指数（穿衣、紫外线、洗车、运动、花粉等）  ## 天气字段说明 `weather` 是天气现象文本，不是固定枚举。  常见值包括：晴、多云、阴、小雨、中雨、大雨、雷阵雨、小雪、中雪、大雪、雨夹雪、雾、霾、沙尘。  如果你的业务需要稳定分类，建议结合 `weather_code` 做自己的映射归类。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**city** | Option<**String**> | 城市名称，支持中文（`北京`）和英文（`Tokyo`）。可选参数，不传时会尝试 IP 自动定位。 |  |
**adcode** | Option<**String**> | 城市行政区划代码（如 `110000`），优先级高于 city。可选参数，不传时会尝试 IP 自动定位。 |  |
**extended** | Option<**bool**> | 返回扩展气象字段（体感温度、能见度、气压、紫外线、降水量、云量、空气质量指数及污染物分项数据）。 |  |
**forecast** | Option<**bool**> | 返回多天预报数据（最多7天），含白天夜间天气、风向风力、日出日落等。 |  |
**hourly** | Option<**bool**> | 返回逐小时预报（24小时），含温度、天气、风向风速、湿度、降水概率等。 |  |
**minutely** | Option<**bool**> | 返回分钟级降水预报（仅国内城市），每5分钟一个数据点，共24个。 |  |
**indices** | Option<**bool**> | 返回18项生活指数（穿衣、紫外线、洗车、晾晒、空调、感冒、运动、舒适度、出行、钓鱼、过敏、防晒、心情、啤酒、雨伞、交通、空气净化器、花粉）。 |  |
**lang** | Option<**String**> | 返回语言。`zh` 返回中文（默认），`en` 返回英文。城市名翻译覆盖 7000+ 城市。生活指数（`indices`）目前仅支持中文。 |  |[default to zh]

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
查询世界时间

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


## post_misc_date_diff

> models::PostMiscDateDiff200Response post_misc_date_diff(post_misc_date_diff_request)
计算两个日期之间的时间差值

想知道两个日期之间相差多久？这个接口帮你精确计算时间差值。  ## 功能概述 输入开始日期和结束日期，返回它们之间的时间差，包括总天数、总小时数、总分钟数、总秒数、总周数，以及人性化显示格式（如\"1年2月3天\"）。  ## 日期格式 接口支持自动识别常见日期格式，包括：YYYY-MM-DD、YYYY/MM/DD、DD-MM-YYYY、ISO 8601（带时区）等。也可以通过`format`参数显式指定格式（如DD-MM-YYYY）。  > [!NOTE] > 当结束日期早于开始日期时，返回的数值为负数。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_misc_date_diff_request** | [**PostMiscDateDiffRequest**](PostMiscDateDiffRequest.md) | 包含日期信息的JSON对象 | [required] |

### Return type

[**models::PostMiscDateDiff200Response**](post_misc_date_diff_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

