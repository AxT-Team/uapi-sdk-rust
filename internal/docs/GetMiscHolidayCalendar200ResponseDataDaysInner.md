# GetMiscHolidayCalendar200ResponseDataDaysInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**date** | Option<**String**> | 公历日期（YYYY-MM-DD）。 | [optional]
**year** | Option<**i32**> | 公历年份。 | [optional]
**month** | Option<**i32**> | 公历月份。 | [optional]
**day** | Option<**i32**> | 公历日期（天）。 | [optional]
**weekday_cn** | Option<**String**> | 中文星期，如星期三。 | [optional]
**is_weekend** | Option<**bool**> | 是否为周末。 | [optional]
**is_workday** | Option<**bool**> | 是否为工作日（含法定调休上班日）。 | [optional]
**is_rest_day** | Option<**bool**> | 是否为休息日。 | [optional]
**is_holiday** | Option<**bool**> | 当天是否存在节日/节气/法定事件。 | [optional]
**legal_holiday_name** | Option<**String**> | 法定节假日名称，无则为空。 | [optional]
**legal_holiday_type** | Option<**String**> | 法定假日类型：rest 或 workday_adjust。 | [optional]
**solar_festival** | Option<**String**> | 公历节日名称，无则为空。 | [optional]
**lunar_festival** | Option<**String**> | 农历节日名称，无则为空。 | [optional]
**solar_term** | Option<**String**> | 节气名称，无则为空。 | [optional]
**lunar_year** | Option<**i32**> | 农历年份（数字）。 | [optional]
**lunar_month** | Option<**i32**> | 农历月份（数字）。 | [optional]
**lunar_day** | Option<**i32**> | 农历日期（数字）。 | [optional]
**lunar_month_name** | Option<**String**> | 农历月份中文名称。 | [optional]
**lunar_day_name** | Option<**String**> | 农历日期中文名称。 | [optional]
**ganzhi_year** | Option<**String**> | 干支年。 | [optional]
**ganzhi_month** | Option<**String**> | 干支月。 | [optional]
**ganzhi_day** | Option<**String**> | 干支日。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


