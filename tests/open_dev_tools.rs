use std::{thread, time};

mod support;

#[tokio::test]
async fn open_dev_tools_test() {
    let _server = support::Process::server();
    let _browser = support::Process::browser(9210);

    thread::sleep(time::Duration::from_millis(2000));

    let websocket_url = cdp::websocket_url_from("http://localhost:9210/json/new").await.unwrap();
    let (write, read) = cdp::connect_to_websocket(websocket_url).await;
    let mut client = cdp::Client::new(write, read).await;
    assert_eq!(client.network().enable(Some(65535)).await, Ok(cdp::network::EnableReturn { __blank: () }));
    // assert_eq!(client.network().set_attach_debug_stack(true).await, cdp::network::SetAttachDebugStackReturn { __blank: () });
    assert_eq!(client.page().enable().await, Ok(cdp::page::EnableReturn { __blank: () }));
    // assert_eq!(client.page().get_resource_tree().await, cdp::page::GetResourceTreeReturn { __blank: () });
    assert_eq!(client.runtime().enable().await, Ok(cdp::runtime::EnableReturn { __blank: () }));
    assert_eq!(client.dom().enable().await, Ok(cdp::dom::EnableReturn { __blank: () }));
    // assert_eq!(client.css().enable().await, cdp::css::EnableReturn { __blank: () });
    let mut ret = client.debugger().enable().await.unwrap();
    ret.debugger_id = "test id".to_owned();
    assert_eq!(ret, cdp::debugger::EnableReturn { debugger_id: "test id".to_owned() });
    assert_eq!(client.debugger().set_pause_on_exceptions("none".to_owned()).await, Ok(cdp::debugger::SetPauseOnExceptionsReturn { __blank: () }));
    assert_eq!(client.debugger().set_async_call_stack_depth(32).await, Ok(cdp::debugger::SetAsyncCallStackDepthReturn { __blank: () }));
    // Overlay.enable	{}	{}
    // Overlay.setShowViewportSizeOnResize	{"show":true}	{}
    // Animation.enable	{}	{}
    // Autofill.enable	{}	{}
    // Autofill.setAddresses	{"addresses":[]}	{}
    assert_eq!(client.profiler().enable().await, Ok(cdp::profiler::EnableReturn { __blank: () }));
    assert_eq!(client.log().enable().await, Ok(cdp::log::EnableReturn { __blank: () }));
    assert_eq!(
        client.log().start_violations_report(
            vec![
                cdp::log::ViolationSetting { name: "longTask".to_owned(), threshold: 200.0 },
                cdp::log::ViolationSetting { name: "longLayout".to_owned(), threshold: 30.0 },
                cdp::log::ViolationSetting { name: "blockedEvent".to_owned(), threshold: 100.0 },
                cdp::log::ViolationSetting { name: "blockedParser".to_owned(), threshold: -1.0 },
                cdp::log::ViolationSetting { name: "handler".to_owned(), threshold: 150.0 },
                cdp::log::ViolationSetting { name: "recurringHandler".to_owned(), threshold: 50.0 },
                cdp::log::ViolationSetting { name: "discouragedAPIUse".to_owned(), threshold: -1.0 },
                
            ]
        ).await,
        Ok(cdp::log::StartViolationsReportReturn { __blank: () })
    );
    // Emulation.setEmulatedMedia	{"media":"","features":[{"name":"color-gamut","value":""},{"name":"prefers-color-scheme","value":""},{"name":"forced-colors","value":""},{"name":"prefers-contrast","value":""},{"name":"prefers-reduced-data","value":""},{"name":"prefers-reduced-motion","value":""},{"name":"prefers-reduced-transparency","value":""}]}	{}
    // Emulation.setEmulatedVisionDeficiency	{"type":"none"}	{}
    // Audits.enable	{}	{}
    // ServiceWorker.enable	{}	{}
    // Inspector.enable	{}	{}
    assert_eq!(client.target().set_auto_attach(true, true).await, Ok(cdp::target::SetAutoAttachReturn { __blank: () }));
    assert_eq!(client.target().set_discover_targets(true).await, Ok(cdp::target::SetDiscoverTargetsReturn { __blank: () }));
    // Target.setRemoteLocations	{"locations":[{"host":"localhost","port":9229}]}	{}
    assert_eq!(client.runtime().add_binding("__chromium_devtools_metrics_reporter".to_owned(), Some("DevTools Performance Metrics".to_owned())).await, Ok(cdp::runtime::AddBindingReturn { __blank: () }));

    // Network.clearAcceptedEncodingsOverride	{}	{}
    // Debugger.setBlackboxPatterns	{"patterns":["/node_modules/|/bower_components/"],"skipAnonymous":false}	{}
    // DOMDebugger.setBreakOnCSPViolation	{"violationTypes":[]}	{}
    // CSS.trackComputedStyleUpdates	{"propertiesToTrack":[{"name":"display","value":"grid"},{"name":"display","value":"inline-grid"},{"name":"display","value":"flex"},{"name":"display","value":"inline-flex"},{"name":"container-type","value":"inline-size"},{"name":"container-type","value":"block-size"},{"name":"container-type","value":"size"}]}	{}
    // CSS.takeComputedStyleUpdates	{}	{"nodeIds":[]}
    let mut ret = client.dom().get_document(None, None).await.unwrap();
    let expected = cdp::dom::GetDocumentReturn {
        root: cdp::dom::Node {
            node_id: 1,
            parent_id: None,
            backend_node_id: 1,
            node_type: 9,
            node_name: "#document".to_owned(),
            local_name: "".to_owned(),
            node_value: "".to_owned(),
            child_node_count: Some(1),
            children: Some(vec![
                cdp::dom::Node {
                    node_id: 2,
                    parent_id: Some(1),
                    backend_node_id: 2,
                    node_type: 1,
                    node_name: "HTML".to_owned(),
                    local_name: "html".to_owned(),
                    node_value: "".to_owned(),
                    child_node_count: Some(2),
                    children: Some(vec![
                        cdp::dom::Node {
                            node_id: 3,
                            parent_id: Some(2),
                            backend_node_id: 3,
                            node_type: 1,
                            node_name: "HEAD".to_owned(),
                            local_name: "head".to_owned(),
                            node_value: "".to_owned(),
                            child_node_count: Some(0),
                            children: None,
                            attributes: Some(vec![]),
                            document_u_r_l: None,
                            base_u_r_l: None,
                            public_id: None,
                            system_id: None,
                            internal_subset: None,
                            xml_version: None,
                            name: None,
                            value: None,
                            pseudo_type: None,
                            pseudo_identifier: None,
                            shadow_root_type: None,
                            frame_id: None,
                            content_document: None,
                            shadow_roots: None,
                            template_content: None,
                            pseudo_elements: None,
                            imported_document: None,
                            distributed_nodes: None,
                            is_s_v_g: None,
                            compatibility_mode: None,
                            assigned_slot: None,
                            is_scrollable: None
                        },
                        cdp::dom::Node {
                            node_id: 4,
                            parent_id: Some(2),
                            backend_node_id: 4,
                            node_type: 1,
                            node_name: "BODY".to_owned(),
                            local_name: "body".to_owned(),
                            node_value: "".to_owned(),
                            child_node_count: Some(0),
                            children: None,
                            attributes: Some(vec![]),
                            document_u_r_l: None,
                            base_u_r_l: None,
                            public_id: None,
                            system_id: None,
                            internal_subset: None,
                            xml_version: None,
                            name: None,
                            value: None,
                            pseudo_type: None,
                            pseudo_identifier: None,
                            shadow_root_type: None,
                            frame_id: None,
                            content_document: None,
                            shadow_roots: None,
                            template_content: None,
                            pseudo_elements: None,
                            imported_document: None,
                            distributed_nodes: None,
                            is_s_v_g: None,
                            compatibility_mode: None,
                            assigned_slot: None,
                            is_scrollable: None
                        }
                    ]),
                    attributes: Some(vec![]),
                    document_u_r_l: None,
                    base_u_r_l: None,
                    public_id: None,
                    system_id: None,
                    internal_subset: None,
                    xml_version: None,
                    name: None,
                    value: None,
                    pseudo_type: None,
                    pseudo_identifier: None,
                    shadow_root_type: None,
                    frame_id: None,
                    content_document: None,
                    shadow_roots: None,
                    template_content: None,
                    pseudo_elements: None,
                    imported_document: None,
                    distributed_nodes: None,
                    is_s_v_g: None,
                    compatibility_mode: None,
                    assigned_slot: None,
                    is_scrollable: None
                }
            ]),
            attributes: None,
            document_u_r_l: Some("about:blank".to_owned()),
            base_u_r_l: Some("about:blank".to_owned()),
            public_id: None,
            system_id: None,
            internal_subset: None,
            xml_version: Some("".to_owned()),
            name: None,
            value: None,
            pseudo_type: None,
            pseudo_identifier: None,
            shadow_root_type: None,
            frame_id: None,
            content_document: None,
            shadow_roots: None,
            template_content: None,
            pseudo_elements: None,
            imported_document: None,
            distributed_nodes: None,
            is_s_v_g: None,
            compatibility_mode: Some(cdp::dom::CompatibilityMode::QuirksMode),
            assigned_slot: None,
            is_scrollable: None
        }
    };

    ret.root.children.as_mut().unwrap()[0].frame_id = None;
    assert_eq!(ret, expected);
    println!("{:?}", client.target().receive_event().await);
    println!("{:?}", client.target().receive_event().await);
    println!("{:?}", client.target().receive_event().await);
    println!("{:?}", client.runtime().receive_event().await);
    // Page.setAdBlockingEnabled	{"enabled":false}	{}
    // Emulation.setFocusEmulationEnabled	{"enabled":false}	{}
}
