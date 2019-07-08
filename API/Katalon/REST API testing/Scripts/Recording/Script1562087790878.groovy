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
import org.openqa.selenium.Keys as Keys

WebUI.click(findTestObject('Object Repository/Recording/Page_Apple (Espaa)/a_Comprar'))

WebUI.click(findTestObject('Object Repository/Recording/Page_Comprar un iPhone XS o iPhone XS Max libre - Apple (ES)/span_Desde 1159'))

WebUI.click(findTestObject('Object Repository/Recording/Page_Comprar un iPhone XS o iPhone XS Max libre - Apple (ES)/input_Elige un modelo_dimensionScreensize'))

WebUI.click(findTestObject('Object Repository/Recording/Page_Comprar un iPhone XS o iPhone XS Max libre - Apple (ES)/div'))

WebUI.click(findTestObject('Object Repository/Recording/Page_Comprar un iPhone XS o iPhone XS Max libre - Apple (ES)/input_Elige la capacidad_dimensionCapacity'))

WebUI.click(findTestObject('Object Repository/Recording/Page_iPhoneXS de 64GB engris espacial - Apple (ES)/span_Continuar'))

WebUI.click(findTestObject('Object Repository/Recording/Page_Garanta - Apple (ES)/button_Quieres aadir la cobertura de AppleCare_form-choice form-choice-selector form-choice-selector-small rounded'))

WebUI.closeBrowser()

