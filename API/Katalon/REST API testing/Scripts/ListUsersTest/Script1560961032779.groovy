import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

WS.sendRequest(findTestObject('UserRestService/List User'))

response = WS.sendRequestAndVerify(findTestObject('UserRestService/List User'))

// HACIENDO LA LLAMADA 'ListUser', EL VALOR 8 DEL ARRAY TENDRÁ first_name=Byron
WS.verifyElementPropertyValue(response, '[8].first_name', 'Byron')

// HACIENDO LA LLAMADA 'ListUser', EL VALOR 10 DEL ARRAY TENDRÁ id=11
WS.verifyElementPropertyValue(response, '[10].id', '11')

// HACIENDO LA LLAMADA 'ListUser', VAN A LLEGAR 20 CAMPOS DE USER
WS.verifyElementsCount(response, 'user', 20)

// COMPRUEBA RANGO DEL CODIGO DE RESPUESTA
WS.verifyResponseStatusCodeInRange(response, 100, 200)

