<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>NewRequestTest</name>
   <tag></tag>
   <elementGuidId>7f51fb2e-9541-443c-920a-ba4fff0b7d59</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:att=&quot;http://www.fastweb.it/PNM/AttivazioneProcessoNotificheSOAP/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;att:attivazioneProcessoNotificheRequest>
         &lt;att:account>
            &lt;att:cognome>Rossi&lt;/att:cognome>
            &lt;att:emailPreferita>spindox.pinot@gmail.com&lt;/att:emailPreferita>
            &lt;att:nome>Mario&lt;/att:nome>
            &lt;att:numeroMobilePreferito>3333333333&lt;/att:numeroMobilePreferito>
            &lt;att:ragioneSociale>Spindox&lt;/att:ragioneSociale>
         &lt;/att:account>
         &lt;att:accountType>FWA&lt;/att:accountType>
         &lt;att:customerId>${#TestCase#CustomerID_Attivo}&lt;/att:customerId>
         &lt;att:headerData>
            &lt;att:correlationId>0000&lt;/att:correlationId>
            &lt;att:correlationType>0000&lt;/att:correlationType>
            &lt;att:requestId>0000&lt;/att:requestId>
            &lt;att:sourceSystem>PNM&lt;/att:sourceSystem>
         &lt;/att:headerData>
         &lt;!--Zero or more repetitions:-->
         &lt;att:parameters>
            &lt;att:key>$data_appuntamento&lt;/att:key>
            &lt;att:value>20-04-2020&lt;/att:value>
         &lt;/att:parameters>
         &lt;att:parameters>
            &lt;att:key>$fascia_appuntamento&lt;/att:key>
            &lt;att:value>こんにちは&lt;/att:value>
         &lt;/att:parameters>
         &lt;att:triggerId>SFDC_conferma_ordine&lt;/att:triggerId>
      &lt;/att:attivazioneProcessoNotificheRequest>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
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
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
