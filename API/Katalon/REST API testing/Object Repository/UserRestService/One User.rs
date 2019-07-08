<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>One User</name>
   <tag></tag>
   <elementGuidId>df1eab2d-cc98-434e-b1f7-3d5ae0e5d9e3</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://localhost:3000/api/user/${user_id_key}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'50'</defaultValue>
      <description></description>
      <id>f77426be-2771-4722-b317-3896f70ad500</id>
      <masked>false</masked>
      <name>user_id_key</name>
   </variables>
   <variables>
      <defaultValue>'Ruben'</defaultValue>
      <description></description>
      <id>6882bb8d-8bc5-4673-94a9-d15210733f0b</id>
      <masked>false</masked>
      <name>first_name_key</name>
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


// PARCEAR JSON
def jsonSlurper = new JsonSlurper()

def jsonResponse = jsonSlurper.parseText(response.getResponseText())


// COMPRUEBA RESPUESTA, EL CAMPO [(first_name) = (['Buckminster'])]
assertThat(jsonResponse.first_name).isEqualTo([first_name_key])
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
