# GetMiscWeather200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**province** | Option<**String**> | 省份 | [optional]
**city** | Option<**String**> | 城市名 | [optional]
**adcode** | Option<**String**> | 行政区划代码（部分数据源可能为空） | [optional]
**weather** | Option<**String**> | 天气状况描述。默认返回中文，传 `lang=en` 时返回英文。非固定枚举。 | [optional]
**temperature** | Option<**f64**> | 当前温度 °C | [optional]
**wind_direction** | Option<**String**> | 风向 | [optional]
**wind_power** | Option<**String**> | 风力等级 | [optional]
**humidity** | Option<**f64**> | 相对湿度 % | [optional]
**report_time** | Option<**String**> | 数据更新时间 | [optional]
**feels_like** | Option<**f64**> | 体感温度 °C（extended=true 时返回） | [optional]
**visibility** | Option<**f64**> | 能见度 km（extended=true 时返回） | [optional]
**pressure** | Option<**f64**> | 气压 hPa（extended=true 时返回） | [optional]
**uv** | Option<**f64**> | 紫外线指数（extended=true 时返回） | [optional]
**precipitation** | Option<**f64**> | 当前降水量 mm（extended=true 时返回） | [optional]
**cloud** | Option<**f64**> | 云量 %（extended=true 时返回） | [optional]
**aqi** | Option<**f64**> | 空气质量指数 0-500（extended=true 时返回） | [optional]
**aqi_level** | Option<**f64**> | AQI 等级 1-6（extended=true 时返回） | [optional]
**aqi_category** | Option<**String**> | AQI 等级描述（优/良/轻度污染/中度污染/重度污染/严重污染）（extended=true 时返回） | [optional]
**aqi_primary** | Option<**String**> | 主要污染物（如 PM2.5、PM10、O3 等）（extended=true 时返回） | [optional]
**air_pollutants** | Option<[**models::GetMiscWeather200ResponseAirPollutants**](get_misc_weather_200_response_air_pollutants.md)> |  | [optional]
**temp_max** | Option<**f64**> | 当天最高温 °C（forecast=true 时返回） | [optional]
**temp_min** | Option<**f64**> | 当天最低温 °C（forecast=true 时返回） | [optional]
**forecast** | Option<[**Vec<models::GetMiscWeather200ResponseForecastInner>**](get_misc_weather_200_response_forecast_inner.md)> | 多天天气预报，最多7天（forecast=true 时返回） | [optional]
**hourly_forecast** | Option<[**Vec<models::GetMiscWeather200ResponseHourlyForecastInner>**](get_misc_weather_200_response_hourly_forecast_inner.md)> | 逐小时预报，最多24小时（hourly=true 时返回） | [optional]
**minutely_precip** | Option<[**models::GetMiscWeather200ResponseMinutelyPrecip**](get_misc_weather_200_response_minutely_precip.md)> |  | [optional]
**life_indices** | Option<[**models::GetMiscWeather200ResponseLifeIndices**](get_misc_weather_200_response_life_indices.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


