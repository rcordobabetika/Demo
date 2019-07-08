<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Edit User Row</name>
   <tag></tag>
   <elementGuidId>6e49b924-3d77-48f9-99ee-c57eb97c361c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;id\&quot;: 41,\n\t\&quot;first_name\&quot;: \&quot;Yolanda\&quot;,\n\t\&quot;phone_number\&quot;: \&quot;(666) 7694\&quot;,\n\t\&quot;email_address\&quot;: \&quot;example@test.com\&quot;,\n\t\&quot;created_date\&quot;: \&quot;2020-06-16 22:33:32\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>http://localhost:3000/api/user/</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


// COMPRUEBA CODIGO RESPUESTA
WS.verifyResponseStatusCode(response, 200)

id_response = '41'
// COMPRUEBA QUE &lt;id> EXISTA EN EL RESPONSE
assertThat(response.getResponseText()).contains('insertId',id_response)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
