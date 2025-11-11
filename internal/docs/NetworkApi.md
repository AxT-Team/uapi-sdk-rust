# \NetworkApi

All URIs are relative to *https://uapis.cn/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_network_dns**](NetworkApi.md#get_network_dns) | **GET** /network/dns | 执行DNS解析查询
[**get_network_icp**](NetworkApi.md#get_network_icp) | **GET** /network/icp | 查询域名ICP备案信息
[**get_network_ipinfo**](NetworkApi.md#get_network_ipinfo) | **GET** /network/ipinfo | 查询指定IP或域名的归属信息
[**get_network_myip**](NetworkApi.md#get_network_myip) | **GET** /network/myip | 获取你的公网IP及归属信息
[**get_network_ping**](NetworkApi.md#get_network_ping) | **GET** /network/ping | 从服务器Ping指定主机
[**get_network_pingmyip**](NetworkApi.md#get_network_pingmyip) | **GET** /network/pingmyip | 从服务器Ping你的客户端IP
[**get_network_portscan**](NetworkApi.md#get_network_portscan) | **GET** /network/portscan | 扫描远程主机的指定端口
[**get_network_urlstatus**](NetworkApi.md#get_network_urlstatus) | **GET** /network/urlstatus | 检查URL的可访问性状态
[**get_network_whois**](NetworkApi.md#get_network_whois) | **GET** /network/whois | 查询域名的WHOIS注册信息
[**get_network_wxdomain**](NetworkApi.md#get_network_wxdomain) | **GET** /network/wxdomain | 检查域名在微信中的访问状态



## get_network_dns

> models::GetNetworkDns200Response get_network_dns(domain, r#type)
执行DNS解析查询

想知道一个域名指向了哪个IP？或者它的邮件服务器是谁？这个接口就像一个在线的 `dig` 或 `nslookup` 工具。  ## 功能概述 你可以查询指定域名的各种DNS记录，包括 `A` (IPv4), `AAAA` (IPv6), `CNAME` (别名), `MX` (邮件交换), `NS` (域名服务器) 和 `TXT` (文本记录)。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** | 你需要查询的域名，例如 'cn.bing.com'。 | [required] |
**r#type** | Option<**String**> | 你想要查询的DNS记录类型。 |  |[default to A]

### Return type

[**models::GetNetworkDns200Response**](get_network_dns_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_network_icp

> models::GetNetworkIcp200Response get_network_icp(domain)
查询域名ICP备案信息

想知道一个网站的背后运营主体是谁吗？如果它是在中国大陆运营的，ICP备案信息可以告诉你答案。  ## 功能概述 提供一个域名，查询其在中国工信部的ICP（Internet Content Provider）备案信息。这对于商业合作前的背景调查、验证网站合法性等场景很有帮助。  > [!NOTE] > **查询范围** > 此查询仅对在中国大陆工信部进行过备案的域名有效。对于国外域名或未备案的域名，将查询不到结果。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** | 需要查询的域名或URL | [required] |

### Return type

[**models::GetNetworkIcp200Response**](get_network_icp_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_network_ipinfo

> models::GetNetworkIpinfo200Response get_network_ipinfo(ip, source)
查询指定IP或域名的归属信息

想知道一个IP地址或域名来自地球的哪个角落？这个接口可以帮你定位它。你可以选择使用默认的GeoIP数据库，也可以指定 `source=commercial` 参数来查询更详细的商业级IP归属信息。  ## 功能概述 提供一个公网IPv4、IPv6地址或域名，我们会利用GeoIP数据库查询并返回它的地理位置（国家、省份、城市）、经纬度、以及所属的运营商（ISP）和自治系统（ASN）信息。这在网络安全分析、访问来源统计等领域非常有用。  当使用 `source=commercial` 参数时，接口将调用高性能商业API，提供更精确的市、区、运营商、时区、海拔等信息。请注意，商业查询的响应时间可能会稍长。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ip** | **String** | 你需要查询的公网IP地址或域名（支持IPv4和IPv6）。 | [required] |
**source** | Option<**String**> | 查询的数据源。如果留空，将使用默认的数据库。如果设置为 `commercial`，将调用商业级API，返回更详细的地理位置信息，但响应时间可能会稍长。 |  |

### Return type

[**models::GetNetworkIpinfo200Response**](get_network_ipinfo_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_network_myip

> models::GetNetworkIpinfo200Response get_network_myip(source)
获取你的公网IP及归属信息

想知道你自己的出口公网IP是多少吗？这个接口就是你的“网络身份证”。你可以选择使用默认的GeoIP数据库，也可以指定 `source=commercial` 参数来查询更详细的商业级IP归属信息。  ## 功能概述 调用此接口，它会返回你（即发起请求的客户端）的公网IP地址，并附带与 `/network/ipinfo` 接口相同的地理位置和网络归属信息。非常适合用于在网页上向用户展示他们自己的IP和地理位置。  当使用 `source=commercial` 参数时，接口将调用高性能商业API，提供更精确的市、区、运营商、时区、海拔等信息。请注意，商业查询的响应时间可能会稍长。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source** | Option<**String**> | 查询的数据源。如果留空，将使用默认的数据库。如果设置为 `commercial`，将调用商业级API，返回更详细的地理位置信息，但响应时间可能会稍长。 |  |

### Return type

[**models::GetNetworkIpinfo200Response**](get_network_ipinfo_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_network_ping

> models::GetNetworkPing200Response get_network_ping(host)
从服务器Ping指定主机

想知道从我们的服务器到你的服务器网络延迟高不高？这个工具可以帮你测试网络连通性。  ## 功能概述 这个接口会从我们的服务器节点对你指定的主机（域名或IP地址）执行 ICMP Ping 操作。它会返回最小、最大、平均延迟以及丢包率等关键指标，是诊断网络问题的得力助手。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**host** | **String** | 你需要 Ping 的目标主机，可以是域名或IP地址。 | [required] |

### Return type

[**models::GetNetworkPing200Response**](get_network_ping_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_network_pingmyip

> models::GetNetworkPingmyip200Response get_network_pingmyip()
从服务器Ping你的客户端IP

这是一个非常方便的快捷接口，想知道你的网络到我们服务器的回程延迟吗？点一下就行！  ## 功能概述 这个接口是 `/network/myip` 和 `/network/ping` 的结合体。它会自动获取你客户端的公网IP，然后从我们的服务器Ping这个IP，并返回延迟数据。这对于快速判断你本地网络到服务器的连接质量非常有用。

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetNetworkPingmyip200Response**](get_network_pingmyip_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_network_portscan

> models::GetNetworkPortscan200Response get_network_portscan(host, port, protocol)
扫描远程主机的指定端口

想检查一下你的服务器上某个端口（比如SSH的22端口或者Web的80端口）是否对外开放？这个工具可以帮你快速确认。  ## 功能概述 你可以指定一个主机和端口号，我们的服务器会尝试连接该端口，并告诉你它是开放的（open）、关闭的（closed）还是超时了（timeout）。这对于网络服务配置检查和基本的安全扫描很有用。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**host** | **String** | 需要扫描的目标主机，可以是域名或IP地址。 | [required] |
**port** | **i32** | 需要扫描的端口号，范围是 1 到 65535。 | [required] |
**protocol** | Option<**String**> | 扫描使用的协议，可以是 'tcp' 或 'udp'。 |  |[default to tcp]

### Return type

[**models::GetNetworkPortscan200Response**](get_network_portscan_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_network_urlstatus

> models::GetNetworkUrlstatus200Response get_network_urlstatus(url)
检查URL的可访问性状态

你的网站或API还好吗？用这个接口给它做个快速“体检”吧。  ## 功能概述 提供一个URL，我们会向它发起一个请求，并返回其HTTP响应状态码。这是一种简单而有效的服务可用性监控方法。  > [!TIP] > **性能优化**：为了提高效率并减少对目标服务器的负载，我们实际发送的是 `HEAD` 请求，而不是 `GET` 请求。`HEAD` 请求只会获取响应头，而不会下载整个页面内容，因此速度更快。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url** | **String** | 你需要检查其可访问性状态的完整URL。 | [required] |

### Return type

[**models::GetNetworkUrlstatus200Response**](get_network_urlstatus_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_network_whois

> models::GetNetworkWhois200Response get_network_whois(domain, format)
查询域名的WHOIS注册信息

想知道一个域名是什么时候注册的、注册商是谁、什么时候到期？WHOIS信息可以告诉你这一切。  ## 功能概述 这是一个在线的WHOIS查询工具。你可以通过如下两种方式获取WHOIS信息：  - **默认行为**（不带参数）：`GET /api/v1/network/whois?domain=google.com`   - 返回一个JSON对象，`whois` 字段为原始、未处理的WHOIS文本字符串。 - **JSON格式化**：`GET /api/v1/network/whois?domain=google.com&format=json`   - 返回一个JSON对象，`whois` 字段为解析后的JSON对象，包含WHOIS信息中的键值对。  这样你既可以获得最全的原始信息，也可以方便地处理结构化数据。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** | 你需要查询WHOIS信息的域名。 | [required] |
**format** | Option<**String**> | 返回格式。留空或为 'text' 时返回原始WHOIS文本，设为 'json' 时返回结构化JSON。 |  |[default to text]

### Return type

[**models::GetNetworkWhois200Response**](get_network_whois_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_network_wxdomain

> models::GetNetworkWxdomain200Response get_network_wxdomain(domain)
检查域名在微信中的访问状态

准备在微信里推广你的网站？最好先检查一下域名是否被“拉黑”了。  ## 功能概述 这个接口可以帮你查询一个域名在微信内置浏览器中的访问状态，即是否被微信封禁。这对于做微信生态推广和营销的开发者来说至关重要。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** | 需要查询的域名。 | [required] |

### Return type

[**models::GetNetworkWxdomain200Response**](get_network_wxdomain_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

