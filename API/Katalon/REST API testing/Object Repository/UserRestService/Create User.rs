<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create User</name>
   <tag></tag>
   <elementGuidId>55014ca8-5df8-4597-ab75-ff7767d7b4c3</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;first_name\&quot;: \&quot;${first_name_key}\&quot;,\n\t\&quot;phone_number\&quot;: \&quot;${phone_number_key}\&quot;,\n\t\&quot;email_address\&quot;: \&quot;${email_address_key}\&quot;,\n\t\&quot;created_date\&quot;: \&quot;${created_date_key}\&quot;\n}&quot;,
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
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://localhost:3000/api/user/</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'Ruben'</defaultValue>
      <description></description>
      <id>0501ce67-92cf-46b4-bc24-3f9eb5d78bdf</id>
      <masked>false</masked>
      <name>first_name_key</name>
   </variables>
   <variables>
      <defaultValue>'1234'</defaultValue>
      <description></description>
      <id>8acee768-1b30-44ce-a2e5-19e1c204dd4e</id>
      <masked>false</masked>
      <name>phone_number_key</name>
   </variables>
   <variables>
      <defaultValue>'email@example.test'</defaultValue>
      <description></description>
      <id>d99dc916-7357-4639-a0ba-8fd28b437a46</id>
      <masked>false</masked>
      <name>email_address_key</name>
   </variables>
   <variables>
      <defaultValue>'2019-07-01 12:38:50'</defaultValue>
      <description></description>
      <id>363f0aed-40e6-414f-8615-0bcd9e4cd8c6</id>
      <masked>false</masked>
      <name>created_date_key</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


// COMPRUEBA CODIGO RESPUESTA, CREACIÓN CORRECTA
WS.verifyResponseStatusCode(response, 200)

// DIFERENCIAS; la 1ª opción es importada
assertThat(response.getStatusCode()).isEqualTo(200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
