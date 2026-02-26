# GetMiscLunartime200ResponseData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**query_timestamp** | Option<**String**> | 原始 ts 入参。 | [optional]
**query_timezone** | Option<**String**> | 原始 timezone 入参。 | [optional]
**timezone** | Option<**String**> | 解析后的时区。 | [optional]
**datetime** | Option<**String**> | 本地化时间，格式 YYYY-MM-DD HH:mm:ss。 | [optional]
**datetime_rfc3339** | Option<**String**> | RFC3339 时间格式。 | [optional]
**timestamp_unix** | Option<**i32**> | 秒级 Unix 时间戳。 | [optional]
**weekday** | Option<**String**> | 星期英文。 | [optional]
**weekday_cn** | Option<**String**> | 星期中文。 | [optional]
**lunar_year** | Option<**i32**> | 农历年份（数字）。 | [optional]
**lunar_month** | Option<**i32**> | 农历月份（数字）。 | [optional]
**lunar_day** | Option<**i32**> | 农历日期（数字）。 | [optional]
**is_leap_month** | Option<**bool**> | 是否闰月。 | [optional]
**lunar_year_cn** | Option<**String**> | 农历年份中文表示。 | [optional]
**lunar_month_cn** | Option<**String**> | 农历月份中文表示。 | [optional]
**lunar_day_cn** | Option<**String**> | 农历日期中文表示。 | [optional]
**ganzhi_year** | Option<**String**> | 干支年。 | [optional]
**ganzhi_month** | Option<**String**> | 干支月。 | [optional]
**ganzhi_day** | Option<**String**> | 干支日。 | [optional]
**zodiac** | Option<**String**> | 生肖。 | [optional]
**solar_term** | Option<**String**> | 节气，无则为空字符串。 | [optional]
**lunar_festivals** | Option<**Vec<String>**> | 农历节日数组。 | [optional]
**solar_festivals** | Option<**Vec<String>**> | 公历节日数组。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


