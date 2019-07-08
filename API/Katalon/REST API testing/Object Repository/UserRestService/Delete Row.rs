<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Delete Row</name>
   <tag></tag>
   <elementGuidId>e769ac0b-01d3-4e77-8044-5af5e50f1d61</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>DELETE</restRequestMethod>
   <restUrl>http://localhost:3000/api/user/${user_key}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'40'</defaultValue>
      <description></description>
      <id>e0322841-4ce2-40b5-bf12-f68531de2cee</id>
      <masked>false</masked>
      <name>user_key</name>
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

// DEFINE
def jsonSlurper = new JsonSlurper()

def jsonResponse = jsonSlurper.parseText(response.getResponseText())


// COMPRUEBA EL CÓDIGO [{issue_key} DE REGISTROS (variable)]
WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)


// COMPRUEBA RESPUESTA, EL CAMPO [(fieldCount) = (0)]
assertThat(jsonResponse.fieldCount).isEqualTo(0)
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
