# PostTextAesEncryptAdvancedRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**text** | **String** | 待加密的明文文本 | 
**key** | **String** | 加密密钥（支持任意长度） | 
**mode** | Option<**String**> | 加密模式：GCM/CBC/ECB/CTR/OFB/CFB（可选，默认GCM） | [optional]
**padding** | Option<**String**> | 填充方式：PKCS7/ZERO/NONE（可选，默认PKCS7） | [optional]
**iv** | Option<**String**> | 自定义IV（可选，Base64编码，16字节）。GCM模式无需此参数 | [optional]
**output_format** | Option<**String**> | 输出格式：base64（默认）或hex | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


