# GetStatusRatelimit200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accepts** | Option<**i32**> | Total number of accepted requests | [optional]
**in_flight** | Option<**i32**> | Number of current in-flight requests | [optional]
**last_update** | Option<**String**> | Last update time of the status | [optional]
**limit** | Option<**i32**> | Current concurrency limit | [optional]
**load** | Option<**f64**> | Calculated system load (in_flight / limit) | [optional]
**min_rtt** | Option<**f64**> | Minimum observed RTT in milliseconds | [optional]
**rejects** | Option<**i32**> | Total number of rejected requests | [optional]
**rtt** | Option<**f64**> | Smoothed RTT in milliseconds | [optional]
**throttled** | Option<**i32**> | Total number of throttled requests | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


