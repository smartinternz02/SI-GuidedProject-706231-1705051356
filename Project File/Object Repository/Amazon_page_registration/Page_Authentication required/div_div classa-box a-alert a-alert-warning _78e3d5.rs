<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_div classa-box a-alert a-alert-warning _78e3d5</name>
   <tag></tag>
   <elementGuidId>961e3605-5f53-4998-9a6f-4d27f6e375ec</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value></value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='a-page']/div[2]</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>fd14a557-538e-4b87-84c1-02e8f3abc9f9</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>a-section</value>
      <webElementGuid>c9050797-405e-48ad-ab41-ab7f56515f9e</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>

&lt;div class=&quot;a-box a-alert a-alert-warning&quot; aria-live=&quot;polite&quot; aria-atomic=&quot;true&quot;>&lt;div class=&quot;a-box-inner a-alert-container&quot;>&lt;h4 class=&quot;a-alert-heading&quot;>JavaScript Is Disabled&lt;/h4>&lt;i class=&quot;a-icon a-icon-alert&quot;>&lt;/i>&lt;div class=&quot;a-alert-content&quot;>This site requires JavaScript to function correctly. Please enable JavaScript on your browser to upload attachments.&lt;/div>&lt;/div>&lt;/div>


    .cvf-aamation-iframe {
        border: none;
        width: 100%;
    }


Sorry, something went wrong on our end. Please try again.
    
        P.when('A', 'mobile-auth-platform', 'cvf-client-side-counters-util', 'acic-component').execute(function(A, mapBridge, clientSideCountersUtil) {
            const $ = A.$;

            // MAP iOS fix supported version: https://w.amazon.com/bin/view/IdentityServices/Mobile/iOS/Documentation/JS_Bridge__Onboarding_Guide_for_Infinite_Spinner
            const mapSupportVersion = Object.freeze({
                major: 6,
                minor: 12,
                patch: 4
            })

            const CounterTypes = Object.freeze({
                AA_DISMISS_SPINNER_API_ERROR: &quot;AADismissSpinnerApiError&quot;,
                AA_MAP_JS_BRIDGE_IS_AVAILABLE: &quot;AAMAPJSBridgeIsAvailable&quot;,
                AA_MAP_DISMISS_SPINNER_TRIGGERED: &quot;AAMAPDismissSpinnerTriggered&quot;,
                AA_INCOMPATIBLE_IOS_APP_VERSION: &quot;AAIncompatibleIOSAppVersion&quot;
            });

            // JS bridge will only be available on app webView.
            if (mapBridge.isJSBridgeAvailable()) {
                clientSideCountersUtil.incrementCounter(CounterTypes.AA_MAP_JS_BRIDGE_IS_AVAILABLE);

                mapBridge.getCurrentAppInfo(function (currentAppInfo) {
                    if (currentAppInfo &amp;&amp; currentAppInfo.platform === &quot;iOS&quot;
                        &amp;&amp; isMAPVersionNewerThanSupportVersion(currentAppInfo.mapVersion)) {
                        clientSideCountersUtil.incrementCounter(CounterTypes.AA_MAP_DISMISS_SPINNER_TRIGGERED);
                        dismissSpinner();
                    } else if (currentAppInfo &amp;&amp; currentAppInfo.platform ===&quot;iOS&quot;) {
                        // Should not render AA challenges for incompatible ios app, will set up sev2 monitor for this error.
                        $('#cvf-aamation-error').removeClass(&quot;cvf-hidden&quot;);
                        clientSideCountersUtil.incrementCounter(CounterTypes.AA_INCOMPATIBLE_IOS_APP_VERSION + &quot;:&quot; + currentAppInfo.mapVersion);
                    }
                });
            }

            acic.setupACIC({
                &quot;data-ref-id&quot;: &quot;cvf&quot;,
                &quot;data-challenge-type&quot;: &quot;ARKOSE_LEVEL_1&quot;,
                &quot;data-callback&quot;: (data) => submitForm(data),
                &quot;data-header-footer&quot;: false,
                &quot;data-context&quot;: '{&quot;sessionId&quot;:&quot;257-0709364-9943213&quot;,&quot;marketplaceId&quot;:&quot;A21TJRUUN4KGV&quot;,&quot;clientUseCase&quot;:&quot;/ap/register&quot;}',
                &quot;data-locale&quot;: &quot;en-IN&quot;,
                &quot;data-iframe-id&quot;: &quot;cvf-aamation-challenge-iframe&quot;,
                &quot;data-host-config&quot;: &quot;prod.EUAmazon&quot;
            });

            function submitForm(data) {
                $('#cvf_aamation_response_token').val(data.sessionToken);
                $('#cvf-aamation-challenge-form').submit();
            }

            function dismissSpinner() {
                setInterval(function () {
                    mapBridge.dismissSpinnerView(function(data) {
                        if (data.error) {
                            $('#cvf-aamation-error').removeClass(&quot;cvf-hidden&quot;);
                            clientSideCountersUtil.incrementCounter(CounterTypes.DISMISS_SPINNER_API_ERROR + &quot;:&quot; + data.error);
                        }
                    });
                }, 3000);
            }

            function isMAPVersionNewerThanSupportVersion(currentVersion) {
                if (!currentVersion) {
                    return false;
                }
                // IOS version: major.minor.patch
                const semanticVersion = currentVersion.split('.');
                const major = parseInt(semanticVersion[0] || &quot;0&quot;);
                const minor = parseInt(semanticVersion[1] || &quot;0&quot;);
                const patch = parseInt(semanticVersion[2] || &quot;0&quot;);

                if (!major || !minor || !patch) {
                    return false;
                }

                if (major !== mapSupportVersion.major) {
                    return major > mapSupportVersion.major;
                }else if (minor !== mapSupportVersion.minor) {
                    return minor > mapSupportVersion.minor;
                }
                return patch >= mapSupportVersion.patch;
            }
        })
    
    






</value>
      <webElementGuid>8e57f9e5-587a-423c-ac84-a99eb2a52740</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;a-page&quot;)/div[@class=&quot;a-section&quot;]</value>
      <webElementGuid>59124100-a22a-451f-8ebb-19791259d811</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='a-page']/div[2]</value>
      <webElementGuid>1dbb2c37-35ef-4fa6-a4bf-caa3702de21a</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div/div[2]</value>
      <webElementGuid>27344755-933f-40d7-92aa-2a10aed1b206</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = concat(&quot;

&lt;div class=&quot;a-box a-alert a-alert-warning&quot; aria-live=&quot;polite&quot; aria-atomic=&quot;true&quot;>&lt;div class=&quot;a-box-inner a-alert-container&quot;>&lt;h4 class=&quot;a-alert-heading&quot;>JavaScript Is Disabled&lt;/h4>&lt;i class=&quot;a-icon a-icon-alert&quot;>&lt;/i>&lt;div class=&quot;a-alert-content&quot;>This site requires JavaScript to function correctly. Please enable JavaScript on your browser to upload attachments.&lt;/div>&lt;/div>&lt;/div>


    .cvf-aamation-iframe {
        border: none;
        width: 100%;
    }


Sorry, something went wrong on our end. Please try again.
    
        P.when(&quot; , &quot;'&quot; , &quot;A&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;mobile-auth-platform&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;cvf-client-side-counters-util&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;acic-component&quot; , &quot;'&quot; , &quot;).execute(function(A, mapBridge, clientSideCountersUtil) {
            const $ = A.$;

            // MAP iOS fix supported version: https://w.amazon.com/bin/view/IdentityServices/Mobile/iOS/Documentation/JS_Bridge__Onboarding_Guide_for_Infinite_Spinner
            const mapSupportVersion = Object.freeze({
                major: 6,
                minor: 12,
                patch: 4
            })

            const CounterTypes = Object.freeze({
                AA_DISMISS_SPINNER_API_ERROR: &quot;AADismissSpinnerApiError&quot;,
                AA_MAP_JS_BRIDGE_IS_AVAILABLE: &quot;AAMAPJSBridgeIsAvailable&quot;,
                AA_MAP_DISMISS_SPINNER_TRIGGERED: &quot;AAMAPDismissSpinnerTriggered&quot;,
                AA_INCOMPATIBLE_IOS_APP_VERSION: &quot;AAIncompatibleIOSAppVersion&quot;
            });

            // JS bridge will only be available on app webView.
            if (mapBridge.isJSBridgeAvailable()) {
                clientSideCountersUtil.incrementCounter(CounterTypes.AA_MAP_JS_BRIDGE_IS_AVAILABLE);

                mapBridge.getCurrentAppInfo(function (currentAppInfo) {
                    if (currentAppInfo &amp;&amp; currentAppInfo.platform === &quot;iOS&quot;
                        &amp;&amp; isMAPVersionNewerThanSupportVersion(currentAppInfo.mapVersion)) {
                        clientSideCountersUtil.incrementCounter(CounterTypes.AA_MAP_DISMISS_SPINNER_TRIGGERED);
                        dismissSpinner();
                    } else if (currentAppInfo &amp;&amp; currentAppInfo.platform ===&quot;iOS&quot;) {
                        // Should not render AA challenges for incompatible ios app, will set up sev2 monitor for this error.
                        $(&quot; , &quot;'&quot; , &quot;#cvf-aamation-error&quot; , &quot;'&quot; , &quot;).removeClass(&quot;cvf-hidden&quot;);
                        clientSideCountersUtil.incrementCounter(CounterTypes.AA_INCOMPATIBLE_IOS_APP_VERSION + &quot;:&quot; + currentAppInfo.mapVersion);
                    }
                });
            }

            acic.setupACIC({
                &quot;data-ref-id&quot;: &quot;cvf&quot;,
                &quot;data-challenge-type&quot;: &quot;ARKOSE_LEVEL_1&quot;,
                &quot;data-callback&quot;: (data) => submitForm(data),
                &quot;data-header-footer&quot;: false,
                &quot;data-context&quot;: &quot; , &quot;'&quot; , &quot;{&quot;sessionId&quot;:&quot;257-0709364-9943213&quot;,&quot;marketplaceId&quot;:&quot;A21TJRUUN4KGV&quot;,&quot;clientUseCase&quot;:&quot;/ap/register&quot;}&quot; , &quot;'&quot; , &quot;,
                &quot;data-locale&quot;: &quot;en-IN&quot;,
                &quot;data-iframe-id&quot;: &quot;cvf-aamation-challenge-iframe&quot;,
                &quot;data-host-config&quot;: &quot;prod.EUAmazon&quot;
            });

            function submitForm(data) {
                $(&quot; , &quot;'&quot; , &quot;#cvf_aamation_response_token&quot; , &quot;'&quot; , &quot;).val(data.sessionToken);
                $(&quot; , &quot;'&quot; , &quot;#cvf-aamation-challenge-form&quot; , &quot;'&quot; , &quot;).submit();
            }

            function dismissSpinner() {
                setInterval(function () {
                    mapBridge.dismissSpinnerView(function(data) {
                        if (data.error) {
                            $(&quot; , &quot;'&quot; , &quot;#cvf-aamation-error&quot; , &quot;'&quot; , &quot;).removeClass(&quot;cvf-hidden&quot;);
                            clientSideCountersUtil.incrementCounter(CounterTypes.DISMISS_SPINNER_API_ERROR + &quot;:&quot; + data.error);
                        }
                    });
                }, 3000);
            }

            function isMAPVersionNewerThanSupportVersion(currentVersion) {
                if (!currentVersion) {
                    return false;
                }
                // IOS version: major.minor.patch
                const semanticVersion = currentVersion.split(&quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot;);
                const major = parseInt(semanticVersion[0] || &quot;0&quot;);
                const minor = parseInt(semanticVersion[1] || &quot;0&quot;);
                const patch = parseInt(semanticVersion[2] || &quot;0&quot;);

                if (!major || !minor || !patch) {
                    return false;
                }

                if (major !== mapSupportVersion.major) {
                    return major > mapSupportVersion.major;
                }else if (minor !== mapSupportVersion.minor) {
                    return minor > mapSupportVersion.minor;
                }
                return patch >= mapSupportVersion.patch;
            }
        })
    
    






&quot;) or . = concat(&quot;

&lt;div class=&quot;a-box a-alert a-alert-warning&quot; aria-live=&quot;polite&quot; aria-atomic=&quot;true&quot;>&lt;div class=&quot;a-box-inner a-alert-container&quot;>&lt;h4 class=&quot;a-alert-heading&quot;>JavaScript Is Disabled&lt;/h4>&lt;i class=&quot;a-icon a-icon-alert&quot;>&lt;/i>&lt;div class=&quot;a-alert-content&quot;>This site requires JavaScript to function correctly. Please enable JavaScript on your browser to upload attachments.&lt;/div>&lt;/div>&lt;/div>


    .cvf-aamation-iframe {
        border: none;
        width: 100%;
    }


Sorry, something went wrong on our end. Please try again.
    
        P.when(&quot; , &quot;'&quot; , &quot;A&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;mobile-auth-platform&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;cvf-client-side-counters-util&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;acic-component&quot; , &quot;'&quot; , &quot;).execute(function(A, mapBridge, clientSideCountersUtil) {
            const $ = A.$;

            // MAP iOS fix supported version: https://w.amazon.com/bin/view/IdentityServices/Mobile/iOS/Documentation/JS_Bridge__Onboarding_Guide_for_Infinite_Spinner
            const mapSupportVersion = Object.freeze({
                major: 6,
                minor: 12,
                patch: 4
            })

            const CounterTypes = Object.freeze({
                AA_DISMISS_SPINNER_API_ERROR: &quot;AADismissSpinnerApiError&quot;,
                AA_MAP_JS_BRIDGE_IS_AVAILABLE: &quot;AAMAPJSBridgeIsAvailable&quot;,
                AA_MAP_DISMISS_SPINNER_TRIGGERED: &quot;AAMAPDismissSpinnerTriggered&quot;,
                AA_INCOMPATIBLE_IOS_APP_VERSION: &quot;AAIncompatibleIOSAppVersion&quot;
            });

            // JS bridge will only be available on app webView.
            if (mapBridge.isJSBridgeAvailable()) {
                clientSideCountersUtil.incrementCounter(CounterTypes.AA_MAP_JS_BRIDGE_IS_AVAILABLE);

                mapBridge.getCurrentAppInfo(function (currentAppInfo) {
                    if (currentAppInfo &amp;&amp; currentAppInfo.platform === &quot;iOS&quot;
                        &amp;&amp; isMAPVersionNewerThanSupportVersion(currentAppInfo.mapVersion)) {
                        clientSideCountersUtil.incrementCounter(CounterTypes.AA_MAP_DISMISS_SPINNER_TRIGGERED);
                        dismissSpinner();
                    } else if (currentAppInfo &amp;&amp; currentAppInfo.platform ===&quot;iOS&quot;) {
                        // Should not render AA challenges for incompatible ios app, will set up sev2 monitor for this error.
                        $(&quot; , &quot;'&quot; , &quot;#cvf-aamation-error&quot; , &quot;'&quot; , &quot;).removeClass(&quot;cvf-hidden&quot;);
                        clientSideCountersUtil.incrementCounter(CounterTypes.AA_INCOMPATIBLE_IOS_APP_VERSION + &quot;:&quot; + currentAppInfo.mapVersion);
                    }
                });
            }

            acic.setupACIC({
                &quot;data-ref-id&quot;: &quot;cvf&quot;,
                &quot;data-challenge-type&quot;: &quot;ARKOSE_LEVEL_1&quot;,
                &quot;data-callback&quot;: (data) => submitForm(data),
                &quot;data-header-footer&quot;: false,
                &quot;data-context&quot;: &quot; , &quot;'&quot; , &quot;{&quot;sessionId&quot;:&quot;257-0709364-9943213&quot;,&quot;marketplaceId&quot;:&quot;A21TJRUUN4KGV&quot;,&quot;clientUseCase&quot;:&quot;/ap/register&quot;}&quot; , &quot;'&quot; , &quot;,
                &quot;data-locale&quot;: &quot;en-IN&quot;,
                &quot;data-iframe-id&quot;: &quot;cvf-aamation-challenge-iframe&quot;,
                &quot;data-host-config&quot;: &quot;prod.EUAmazon&quot;
            });

            function submitForm(data) {
                $(&quot; , &quot;'&quot; , &quot;#cvf_aamation_response_token&quot; , &quot;'&quot; , &quot;).val(data.sessionToken);
                $(&quot; , &quot;'&quot; , &quot;#cvf-aamation-challenge-form&quot; , &quot;'&quot; , &quot;).submit();
            }

            function dismissSpinner() {
                setInterval(function () {
                    mapBridge.dismissSpinnerView(function(data) {
                        if (data.error) {
                            $(&quot; , &quot;'&quot; , &quot;#cvf-aamation-error&quot; , &quot;'&quot; , &quot;).removeClass(&quot;cvf-hidden&quot;);
                            clientSideCountersUtil.incrementCounter(CounterTypes.DISMISS_SPINNER_API_ERROR + &quot;:&quot; + data.error);
                        }
                    });
                }, 3000);
            }

            function isMAPVersionNewerThanSupportVersion(currentVersion) {
                if (!currentVersion) {
                    return false;
                }
                // IOS version: major.minor.patch
                const semanticVersion = currentVersion.split(&quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot;);
                const major = parseInt(semanticVersion[0] || &quot;0&quot;);
                const minor = parseInt(semanticVersion[1] || &quot;0&quot;);
                const patch = parseInt(semanticVersion[2] || &quot;0&quot;);

                if (!major || !minor || !patch) {
                    return false;
                }

                if (major !== mapSupportVersion.major) {
                    return major > mapSupportVersion.major;
                }else if (minor !== mapSupportVersion.minor) {
                    return minor > mapSupportVersion.minor;
                }
                return patch >= mapSupportVersion.patch;
            }
        })
    
    






&quot;))]</value>
      <webElementGuid>45c9322b-2609-4379-aa2f-a999f4d08053</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
