use std::{thread, time};

mod support;

use cdp::dom::*;

#[tokio::test]
async fn test_dom() {
    let mut server = support::Process::server();
    let mut browser = support::Process::browser(9210);

    thread::sleep(time::Duration::from_millis(2000));

    // can read stdout for ws://127.0.0.1:9210/devtools/browser/baab0c19-568b-4d95-aa8a-9fc07fb86ff2
    let websocket_url = cdp::websocket_url_from("http://localhost:9210/json/new").await.unwrap();
    let (write, read) = cdp::connect_to_websocket(websocket_url).await;
    let mut client = cdp::Client::new(write, read).await;
    let mut target = client.target();
    target
        .create_target(
            "http://localhost:3000".to_owned(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await;

    let mut actual = client.dom().get_document(None, None).await;

    let expected = GetDocumentReturn {
        root: Node {
            node_id: 1,
            parent_id: None,
            backend_node_id: 1,
            node_type: 9,
            node_name: "#document".to_owned(),
            local_name: "".to_owned(),
            node_value: "".to_owned(),
            child_node_count: Some(1),
            children: Some(vec![
                Node {
                    node_id: 2,
                    parent_id: Some(1),
                    backend_node_id: 2,
                    node_type: 1,
                    node_name: "HTML".to_owned(),
                    local_name: "html".to_owned(),
                    node_value: "".to_owned(),
                    child_node_count: Some(2),
                    children: Some(vec![
                        Node {
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
                        Node {
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
            compatibility_mode: Some(CompatibilityMode::QuirksMode),
            assigned_slot: None,
            is_scrollable: None
        }
    };

    actual.root.children.as_mut().unwrap()[0].frame_id = None;
    assert_eq!(actual, expected);
}
