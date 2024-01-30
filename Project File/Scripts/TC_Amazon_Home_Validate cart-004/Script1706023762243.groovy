import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://www.amazon.in/')

WebUI.selectOptionByValue(findTestObject('Object Repository/Amazon_Pages_AddCard_OR/Page_Online Shopping site in India Shop Onl_10c5f3/select_All Categories        Alexa Skills  _a62561'), 
    'search-alias=stripbooks', true)

WebUI.setText(findTestObject('Object Repository/Amazon_Pages_AddCard_OR/Page_Online Shopping site in India Shop Onl_10c5f3/input_Search Amazon.in_field-keywords'), 
    'dan vinci')

WebUI.click(findTestObject('Object Repository/Amazon_Pages_AddCard_OR/Page_Online Shopping site in India Shop Onl_10c5f3/span_da vinci code'))

CustomKeywords.'custom_key.keyword.addcard'()

/*WebUI.click(findTestObject('Object Repository/Amazon_Pages_AddCard_OR/Page_Amazon.in  da vinci code/img_Shop now__bXVsd_image_1pfbQ'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Amazon_Pages_AddCard_OR/Page_The Khandavaprastha Conspiracy a fast _e5b672/select_1        2        3        4        _45ea30'), 
    '4', true)

WebUI.click(findTestObject('Object Repository/Amazon_Pages_AddCard_OR/Page_The Khandavaprastha Conspiracy a fast _e5b672/input_Quantity_submit.add-to-cart'))*/
WebUI.closeBrowser()

