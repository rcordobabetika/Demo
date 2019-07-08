<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>NO_Exist Confirmation</name>
   <tag></tag>
   <elementGuidId>c4127c80-8907-470c-9ff2-f86e1a5bc23a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://localhost:3000/api/user/${user_id}/exists</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'50'</defaultValue>
      <description>&lt;NUMBER USER> id</description>
      <id>385a18d8-e50e-4391-a054-ba4793d102f2</id>
      <masked>false</masked>
      <name>user_id</name>
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


// COMPRUEBA CODIGO RESPUESTA (HA DE FALLAR)
WS.verifyResponseStatusCode(response, 304)

assertThat(response.getStatusCode()).isEqualTo(304)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
