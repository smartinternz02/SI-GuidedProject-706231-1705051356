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

WebUI.click(findTestObject('Object Repository/Amazon_page_registration/Page_Amazon Sign In/a_Create your Amazon account'))

WebUI.setText(findTestObject('Object Repository/Amazon_page_registration/Page_Amazon Registration/input_Your name_customerName'), 
    'Priya More')

WebUI.setText(findTestObject('Object Repository/Amazon_page_registration/Page_Amazon Registration/input_IN 91_email'), '9011474025')

WebUI.setEncryptedText(findTestObject('Object Repository/Amazon_page_registration/Page_Amazon Registration/input_Password_password'), 
    'Q1r2Exj96yutof03rfSzoA==')

WebUI.click(findTestObject('Object Repository/Amazon_page_registration/Page_Amazon Registration/input_Passwords must be at least 6 characte_167931'))

WebUI.click(findTestObject('Object Repository/Amazon_page_registration/Page_Authentication required/div_div classa-box a-alert a-alert-warning _78e3d5'))

WebUI.closeBrowser()

