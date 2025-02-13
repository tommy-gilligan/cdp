use codegen::Scope;

fn parse_ref(p: &crate::parser::Parameter) -> Option<String> {
    let name = p.r#ref.clone().unwrap();

    match name.split_once(".") {
        // we should expect a flat structure (no domains within domains)
        // maybe should use something more precise than 'super'
        Some((module, r#struct)) => Some(format!(
            "super::{}::{}",
            field_name(module.to_owned()),
            r#struct
        )),
        None => Some(name),
    }
}

use regex::{Captures, Regex};

fn process_doc(s: String) -> String {
    let re = Regex::new(r"(\s)(<[^>]+>)").unwrap();
    let a = re
        .replace_all(&s, |caps: &Captures| format!("{}`{}`", &caps[1], &caps[2]))
        .into_owned();

    let re = Regex::new(r"(https?://\S+)").unwrap();
    let b = re
        .replace_all(&a, |caps: &Captures| format!("<{}>", &caps[1]))
        .into_owned();

    let re = Regex::new(r"(\[|\])").unwrap();
    re.replace_all(&b, |caps: &Captures| format!("\\{}", &caps[1]))
        .into_owned()
}

fn to_type(p: &crate::parser::Parameter, self_referential: bool) -> Option<(codegen::Type, bool)> {
    let l = match p.r#type.clone() {
        None if p.r#ref.is_some() => parse_ref(p),
        Some(_s) if p.r#type.clone() == Some("array".to_owned()) => {
            if let Some(items) = &p.items {
                if let Some(name) = items.r#ref.clone() {
                    match name.split_once(".") {
                        // we should expect a flat structure (no domains within domains)
                        // maybe should use something more precise than 'super'
                        Some((module, r#struct)) => Some(format!(
                            "Vec<super::{}::{}>",
                            field_name(module.to_owned()),
                            r#struct
                        )),
                        None => Some(format!("Vec<{}>", name).to_owned()),
                    }
                } else if let Some(name) = items.r#type.clone() {
                    match name.as_str() {
                        "string" => Some("Vec<String>".to_owned()),
                        "integer" => Some("Vec<u64>".to_owned()),
                        "number" => Some("Vec<f64>".to_owned()),
                        "boolean" => Some("Vec<bool>".to_owned()),
                        "object" => Some("Vec<serde_json::value::Value>".to_owned()),
                        "any" => Some("Vec<()>".to_owned()),
                        _ => {
                            panic!("{:?}", items);
                        }
                    }
                } else {
                    panic!("{:?}", items);
                }
            } else {
                None
            }
        }
        Some(_s) if p.r#type.clone() == Some("string".to_owned()) => Some("String".to_owned()),
        Some(_s) if p.r#type.clone() == Some("integer".to_owned()) => Some("u64".to_owned()),
        Some(_s) if p.r#type.clone() == Some("number".to_owned()) => Some("f64".to_owned()),
        Some(_s) if p.r#type.clone() == Some("boolean".to_owned()) => Some("bool".to_owned()),
        Some(_s) if p.r#type.clone() == Some("any".to_owned()) => Some("()".to_owned()),
        Some(_s) if p.r#type.clone() == Some("object".to_owned()) => {
            Some("serde_json::value::Value".to_owned())
        }
        Some(s) => panic!("{:?}", s),
        _ => None,
    };

    if let Some(r) = l {
        let c = match (p.optional, self_referential) {
            (Some(true), true) => (codegen::Type::new(format!("Option<Box<{}>>", r)), true),
            (Some(true), false) => (codegen::Type::new(format!("Option<{}>", r)), true),
            (_, true) => (codegen::Type::new(format!("Box<{}>", r)), false),
            (_, false) => (codegen::Type::new(r), false),
        };
        return Some(c);
    }
    None
}

fn escape(field_name: String) -> String {
    field_name.replace("-", "_")
}

fn escape_field_name(field_name: String) -> String {
    match field_name.as_str() {
        "type" => "r#type".to_owned(),
        "continue" => "r#continue".to_owned(),
        "override" => "r#override".to_owned(),
        // Felf: FakeSelf
        "Self" => "r#Felf".to_owned(),
        _ => field_name,
    }
}

fn variant_name(name: String) -> String {
    let mut result = String::new();
    let mut upper = false;
    for (index, c) in name.chars().enumerate() {
        if index == 0 {
            if c.is_lowercase() {
                result.push(c.to_ascii_uppercase());
            } else {
                result.push(c);
            }
        } else if c == '_' || c == '-' {
            upper = true;
        } else if upper {
            upper = false;
            let uppercase: char = c.to_ascii_uppercase();
            result.push(uppercase);
        } else {
            result.push(c);
        }
    }
    result
}

pub fn field_name(name: String) -> String {
    let tr = match name.clone().as_str() {
        "backendDOMNodeId" => "backend_dom_node_id".to_owned(),
        "DOM" => "dom".to_owned(),
        "DOMDebugger" => "dom_debugger".to_owned(),
        "DOMSnapshot" => "dom_snapshot".to_owned(),
        "DOMStorage" => "dom_storage".to_owned(),
        "IO" => "io".to_owned(),
        "PWA" => "pwa".to_owned(),
        "IndexedDB" => "indexed_db".to_owned(),
        "CSS" => "css".to_owned(),
        "printToPDF" => "print_to_pdf".to_owned(),
        "setBypassCSP" => "set_bypass_csp".to_owned(),
        "setCPUThrottlingRate" => "set_cpu_throttling_rate".to_owned(),
        "setXHRBreakpoint" => "set_xhr_breakpoint".to_owned(),
        "setShowFPSCounter" => "set_show_fps_counter".to_owned(),
        "getDOMCountersForLeakDetection" => "get_dom_counters_for_leak_detection".to_owned(),
        "getDOMCounters" => "get_dom_counters".to_owned(),
        "getDOMStorageItems" => "get_dom_storage_items".to_owned(),
        "getOuterHTML" => "get_outer_html".to_owned(),
        "removeXHRBreakpoint" => "remove_xhr_breakpoint".to_owned(),
        "trackIndexedDBForOrigin" => "track_indexed_db_for_origin".to_owned(),
        "trackIndexedDBForStorageKey" => "track_indexed_db_for_storage_key".to_owned(),
        "setOuterHTML" => "set_outer_html".to_owned(),
        "setExtraHTTPHeaders" => "set_extra_http_headers".to_owned(),
        "baseURL" => "base_url".to_owned(),
        "blockedURL" => "blocked_url".to_owned(),
        "currentSourceURL" => "current_source_url".to_owned(),
        "dataURL" => "data_url".to_owned(),
        "documentURL" => "document_url".to_owned(),
        "enableUI" => "enable_ui".to_owned(),
        "externalURL" => "external_url".to_owned(),
        "gatedAPIFeatures" => "gated_api_features".to_owned(),
        "hasSourceURL" => "has_source_url".to_owned(),
        "includeCommandLineAPI" => "include_command_line_api".to_owned(),
        "includeDOMRects" => "include_dom_rects".to_owned(),
        "includeObjectsCollectedByMajorGC" => "include_objects_collected_by_major_gc".to_owned(),
        "includeObjectsCollectedByMinorGC" => "include_objects_collected_by_minor_gc".to_owned(),
        "includeUserAgentShadowDOM" => "include_user_agent_shadow_dom".to_owned(),
        "initiatorIPAddressSpace" => "initiator_ip_address_space".to_owned(),
        "insecureURL" => "insecure_url".to_owned(),
        "isBadUP" => "is_bad_up".to_owned(),
        "isBadUV" => "is_bad_uv".to_owned(),
        "isSVG" => "is_svg".to_owned(),
        "mainResourceURL" => "main_resource_url".to_owned(),
        "matchedCSSRules" => "matched_css_rules".to_owned(),
        "modernSSL" => "modern_ssl".to_owned(),
        "originURL" => "origin_url".to_owned(),
        "outerHTML" => "outer_html".to_owned(),
        "preferCSSPageSize" => "prefer_css_page_size".to_owned(),
        "remoteIPAddress" => "remote_ip_address".to_owned(),
        "removeDOMBreakpoint" => "remove_dom_breakpoint".to_owned(),
        "removeDOMStorageItem" => "remove_dom_storage_item".to_owned(),
        "reportAAA" => "report_aaa".to_owned(),
        "requestURL" => "request_url".to_owned(),
        "resourceIPAddressSpace" => "resource_ip_address_space".to_owned(),
        "scopeURL" => "scope_url".to_owned(),
        "scriptURL" => "script_url".to_owned(),
        "setDOMBreakpoint" => "set_dom_breakpoint".to_owned(),
        "setDOMStorageItem" => "set_dom_storage_item".to_owned(),
        "showCSS" => "show_css".to_owned(),
        "sourceMapURL" => "source_map_url".to_owned(),
        "sourceURL" => "source_url".to_owned(),
        "thresholdAA" => "threshold_aa".to_owned(),
        "thresholdAAA" => "threshold_aaa".to_owned(),
        "untrackIndexedDBForOrigin" => "untrack_indexed_db_for_origin".to_owned(),
        "untrackIndexedDBForStorageKey" => "untrack_indexed_db_for_storage_key".to_owned(),
        "userTypedURL" => "user_typed_url".to_owned(),
        _ => {
            let mut result = String::new();
            for (index, c) in name.chars().enumerate() {
                if index == 0 {
                    let lowercase: char = c.to_ascii_lowercase();
                    result.push(lowercase);
                } else if c.is_uppercase() {
                    result.push('_');
                    let lowercase: char = c.to_ascii_lowercase();
                    result.push(lowercase);
                } else {
                    result.push(c);
                }
            }
            result
        }
    };

    println!("{}", name);
    println!("{}", tr);

    tr
}

pub fn main_client(domains: &[crate::parser::Domain]) -> Scope {
    let mut scope = Scope::new();
    let new_trait = scope.new_trait("DomainClients");
    new_trait.vis("pub");

    for domain in domains {
        let function = new_trait.new_fn(field_name(domain.domain.to_owned()));
        function.arg_mut_self();

        let domain_client = format!("{}::Client::<Self>", field_name(domain.domain.to_owned()));
        function.ret(&domain_client);
        function.bound("Self", "crate::Client");
        function.bound("Self", "Sized");

        function.line(format!("{} {}", &domain_client, "{ inner: self }"));
    }

    scope
}

pub fn modules(domains: &[crate::parser::Domain]) -> String {
    let mut s = String::new();
    for domain in domains {
        s.push_str(&format!(
            "pub mod {};\n",
            field_name(domain.domain.to_owned())
        ));
    }
    s
}

pub fn r#gen(domain: crate::parser::Domain) -> Scope {
    let mut module = Scope::new();

    if let Some(ref types) = domain.types {
        for t in types {
            match t.r#type.as_str() {
                "object" => {
                    if let Some(e) = &t.r#properties {
                        let s = module.new_struct(variant_name(t.id.clone()));
                        s.derive("Debug");
                        s.derive("PartialEq");
                        s.derive("crate::Deserialize");
                        s.derive("crate::Serialize");
                        s.derive("Clone");

                        for v in e {
                            if let Some((c, _)) =
                                to_type(v, v.r#ref.clone().unwrap_or("".to_owned()) == t.id)
                            {
                                let g =
                                    s.new_field(field_name(escape_field_name(v.name.clone())), c);

                                g.annotation(format!("#[serde(rename = \"{}\")]", v.name.clone()));

                                g.vis("pub");
                                if let Some(description) = &v.description {
                                    g.doc(process_doc(description.clone()));
                                };
                            }
                        }

                        s.vis("pub");
                        if let Some(description) = &t.description {
                            s.doc(process_doc(description.clone()));
                        };
                    } else {
                        let s = module
                            .new_type_alias(variant_name(t.id.clone()), "serde_json::value::Value");

                        s.vis("pub");
                        if let Some(description) = &t.description {
                            s.doc(process_doc(description.clone()));
                        };
                    }
                }
                "string" => {
                    if let Some(e) = &t.r#enum {
                        let s = module.new_enum(&t.id);
                        s.derive("Debug");
                        s.derive("PartialEq");
                        s.derive("Clone");
                        s.derive("crate::Deserialize");
                        s.derive("crate::Serialize");

                        for v in e {
                            s.new_variant(escape_field_name(variant_name(escape(v.to_string()))))
                                .annotation(format!("#[serde(rename = \"{}\")]", v));
                        }

                        s.vis("pub");
                        if let Some(description) = &t.description {
                            s.doc(process_doc(description.clone()));
                        };
                    } else {
                        let s = module.new_type_alias(&t.id, "String");
                        s.vis("pub");
                        if let Some(description) = &t.description {
                            s.doc(process_doc(description.clone()));
                        };
                    }
                }
                "boolean" => {
                    let s = module.new_type_alias(&t.id, "bool");
                    s.vis("pub");
                    if let Some(description) = &t.description {
                        s.doc(process_doc(description.clone()));
                    };
                }
                "integer" => {
                    let s = module.new_type_alias(&t.id, "u64");
                    s.vis("pub");
                    if let Some(description) = &t.description {
                        s.doc(process_doc(description.clone()));
                    };
                }
                "number" => {
                    let s = module.new_type_alias(&t.id, "f64");
                    s.vis("pub");
                    if let Some(description) = &t.description {
                        s.doc(process_doc(description.clone()));
                    };
                }
                "array" => {
                    let i = if let Some(items) = &t.items {
                        if let Some(name) = items.r#ref.clone() {
                            match name.split_once(".") {
                                // we should expect a flat structure (no domains within domains)
                                // maybe should use something more precise than 'super'
                                Some((module, r#struct)) => Some(format!(
                                    "Vec<super::{}::{}>",
                                    field_name(module.to_owned()),
                                    r#struct
                                )),
                                None => Some(format!("Vec<{}>", name).to_owned()),
                            }
                        } else if let Some(name) = items.r#type.clone() {
                            match name.as_str() {
                                "string" => Some("Vec<String>".to_owned()),
                                "integer" => Some("Vec<u64>".to_owned()),
                                "number" => Some("Vec<f64>".to_owned()),
                                "any" => Some("Vec<()>".to_owned()),
                                _ => {
                                    panic!("{:?}", items);
                                }
                            }
                        } else {
                            panic!("{:?}", items);
                        }
                    } else {
                        None
                    };

                    let s = module.new_type_alias(&t.id, i.unwrap());
                    s.vis("pub");
                    if let Some(description) = &t.description {
                        s.doc(process_doc(description.clone()));
                    };
                }
                t => {
                    unimplemented!("type {} unsupported", t);
                }
            }
        }
    }

    for t in &domain.commands {
        let g = module.new_struct(variant_name(t.name.clone()));

        g.derive("Debug");
        g.derive("PartialEq");
        g.derive("crate::Deserialize");
        g.derive("crate::Serialize");
        g.vis("pub");

        if let Some(description) = &t.description {
            g.doc(process_doc(description.clone()));
        };

        if let Some(parameters) = &t.parameters {
            for p in parameters {
                if let Some((c, o)) = to_type(p, false) {
                    let q = g
                        .new_field(field_name(escape_field_name(p.name.clone())), c)
                        .vis("pub")
                        .annotation(format!("#[serde(rename = \"{}\")]", p.name.clone()));
                    if o {
                        q.annotation("#[serde(skip_serializing_if = \"Option::is_none\")]");
                    }
                }
            }
        }

        if let Some(returns) = &t.returns {
            let result_name = format!("{}Return", variant_name(t.name.clone()));
            let r = module.new_struct(&result_name);
            r.derive("Debug");
            r.derive("PartialEq");
            r.derive("crate::Deserialize");
            r.derive("crate::Serialize");
            r.vis("pub");

            for a_return in returns {
                if let Some((c, _)) = to_type(a_return, false) {
                    r.new_field(field_name(escape_field_name(a_return.name.clone())), c)
                        .vis("pub")
                        .annotation(format!("#[serde(rename = \"{}\")]", a_return.name.clone()));
                }
            }

            module
                .new_impl(variant_name(t.name.clone()))
                .impl_trait("crate::CommandTrait")
                .associate_type("Result", result_name);
        } else {
            let result_name = format!("{}Return", variant_name(t.name.clone()));
            let r = module.new_struct(&result_name);
            r.derive("Debug");
            r.derive("PartialEq");
            r.derive("crate::Deserialize");
            r.derive("crate::Serialize");
            r.vis("pub");
            r.new_field("__blank", "()")
                .vis("pub")
                .annotation("#[serde(skip)]".to_string());

            module
                .new_impl(variant_name(t.name.clone()))
                .impl_trait("crate::CommandTrait")
                .associate_type("Result", result_name);
        }
    }

    let client = module.new_struct("Client");
    client.new_field("inner", "&'a mut T").vis("pub");

    client
        .vis("pub")
        .generic("'a")
        .generic("T")
        .bound("T", "crate::Client");

    let client_impl = module.new_impl("Client");
    client_impl.target_generic("'_");
    client_impl.generic("T");
    client_impl.target_generic("T");
    client_impl.bound("T", "crate::Client");

    for t in &domain.commands {
        let function = client_impl.new_fn(field_name(escape_field_name(t.name.clone())));
        function.vis("pub");
        function.arg_mut_self();
        function.set_async(true);
        function.ret(format!(
            "Result<{}Return, (isize, String)>",
            variant_name(t.name.clone())
        ));

        if let Some(description) = &t.description {
            function.doc(process_doc(description.clone()));
        };

        let mut args: Vec<String> = Vec::new();

        if let Some(parameters) = &t.parameters {
            for p in parameters {
                if let Some((c, _o)) = to_type(p, false) {
                    let name = field_name(escape_field_name(p.name.clone()));
                    args.push(name.clone());
                    function.arg(&name, c);
                }
            }
        }

        let mut s = "{".to_owned();
        s.push_str(&args.join(", "));
        s.push('}');

        function.line(format!(
            "let request = {} {};",
            &variant_name(t.name.clone()),
            s
        ));
        function.line(format!(
            "self.inner.send_command(\"{}.{}\", request).await",
            domain.domain.to_owned(),
            t.name.clone()
        ));
    }

    let function = client_impl.new_fn("receive_event");
    function.vis("pub");
    function.arg_mut_self();
    function.set_async(true);
    function.ret("Event");
    function.line("self.inner.receive_event().await");

    if let Some(ref events) = domain.events {
        for t in events {
            let g = module.new_struct(variant_name(t.name.clone()));

            g.derive("Debug");
            g.derive("Clone");
            g.derive("PartialEq");
            g.derive("crate::Deserialize");
            g.derive("crate::Serialize");
            g.vis("pub");

            if let Some(description) = &t.description {
                g.doc(process_doc(description.clone()));
            };

            if let Some(parameters) = &t.parameters {
                for p in parameters {
                    if let Some((c, o)) = to_type(p, false) {
                        let q = g
                            .new_field(field_name(escape_field_name(p.name.clone())), c)
                            .vis("pub")
                            .annotation(format!("#[serde(rename = \"{}\")]", p.name.clone()));
                        if o {
                            q.annotation("#[serde(skip_serializing_if = \"Option::is_none\")]");
                        }
                    }
                }
            }
        }
    }

    let g = module
        .new_enum(variant_name("Event".to_owned()))
        .derive("Debug")
        .derive("PartialEq")
        .derive("crate::Deserialize")
        .derive("crate::Serialize")
        .derive("Clone")
        .vis("pub")
        .r#macro("#[serde(tag = \"method\", content = \"params\")]");

    if let Some(ref events) = domain.events {
        for t in events {
            let variant =
                g.new_variant(escape_field_name(variant_name(escape(t.name.to_string()))));
            if let Some(description) = &t.description {
                for line in description.lines() {
                    variant.annotation(format!("/// {}", line));
                }
            }
            variant
                .annotation(format!(
                    "#[serde(rename = \"{}.{}\")]",
                    domain.domain, t.name
                ))
                .tuple(variant_name(t.name.clone()));
        }
    }

    module
}

#[test]
fn test_field_name() {
    assert_eq!(
        field_name("getDOMCountersForLeakDetection".to_owned()),
        "get_dom_counters_for_leak_detection".to_owned()
    );
    assert_eq!(
        field_name("getDOMCounters".to_owned()),
        "get_dom_counters".to_owned()
    );
    assert_eq!(
        field_name("getDOMStorageItems".to_owned()),
        "get_dom_storage_items".to_owned()
    );
    assert_eq!(
        field_name("getOuterHTML".to_owned()),
        "get_outer_html".to_owned()
    );
    assert_eq!(
        field_name("removeXHRBreakpoint".to_owned()),
        "remove_xhr_breakpoint".to_owned()
    );
    assert_eq!(
        field_name("trackIndexedDBForOrigin".to_owned()),
        "track_indexed_db_for_origin".to_owned()
    );
    assert_eq!(
        field_name("trackIndexedDBForStorageKey".to_owned()),
        "track_indexed_db_for_storage_key".to_owned()
    );
    assert_eq!(
        field_name("printToPDF".to_owned()),
        "print_to_pdf".to_owned()
    );
    assert_eq!(
        field_name("setBypassCSP".to_owned()),
        "set_bypass_csp".to_owned()
    );
    assert_eq!(
        field_name("setCPUThrottlingRate".to_owned()),
        "set_cpu_throttling_rate".to_owned()
    );
    assert_eq!(
        field_name("setXHRBreakpoint".to_owned()),
        "set_xhr_breakpoint".to_owned()
    );
    assert_eq!(
        field_name("setShowFPSCounter".to_owned()),
        "set_show_fps_counter".to_owned()
    );
    assert_eq!(
        field_name("setOuterHTML".to_owned()),
        "set_outer_html".to_owned()
    );
    assert_eq!(
        field_name("setExtraHTTPHeaders".to_owned()),
        "set_extra_http_headers".to_owned()
    );
    assert_eq!(
        field_name("backendDOMNodeId".to_owned()),
        "backend_dom_node_id".to_owned(),
    );
    assert_eq!(field_name("baseURL".to_owned()), "base_url".to_owned(),);
    assert_eq!(
        field_name("blockedURL".to_owned()),
        "blocked_url".to_owned(),
    );
    assert_eq!(
        field_name("currentSourceURL".to_owned()),
        "current_source_url".to_owned(),
    );
    assert_eq!(field_name("dataURL".to_owned()), "data_url".to_owned(),);
    assert_eq!(
        field_name("documentURL".to_owned()),
        "document_url".to_owned(),
    );
    assert_eq!(field_name("enableUI".to_owned()), "enable_ui".to_owned(),);
    assert_eq!(
        field_name("externalURL".to_owned()),
        "external_url".to_owned(),
    );
    assert_eq!(
        field_name("gatedAPIFeatures".to_owned()),
        "gated_api_features".to_owned(),
    );
    assert_eq!(
        field_name("hasSourceURL".to_owned()),
        "has_source_url".to_owned(),
    );
    assert_eq!(
        field_name("includeCommandLineAPI".to_owned()),
        "include_command_line_api".to_owned(),
    );
    assert_eq!(
        field_name("includeDOMRects".to_owned()),
        "include_dom_rects".to_owned(),
    );
    assert_eq!(
        field_name("includeObjectsCollectedByMajorGC".to_owned()),
        "include_objects_collected_by_major_gc".to_owned(),
    );
    assert_eq!(
        field_name("includeObjectsCollectedByMinorGC".to_owned()),
        "include_objects_collected_by_minor_gc".to_owned(),
    );
    assert_eq!(
        field_name("includeUserAgentShadowDOM".to_owned()),
        "include_user_agent_shadow_dom".to_owned(),
    );
    assert_eq!(
        field_name("initiatorIPAddressSpace".to_owned()),
        "initiator_ip_address_space".to_owned(),
    );
    assert_eq!(
        field_name("insecureURL".to_owned()),
        "insecure_url".to_owned(),
    );
    assert_eq!(field_name("isBadUP".to_owned()), "is_bad_up".to_owned(),);
    assert_eq!(field_name("isBadUV".to_owned()), "is_bad_uv".to_owned(),);
    assert_eq!(field_name("isSVG".to_owned()), "is_svg".to_owned(),);
    assert_eq!(
        field_name("mainResourceURL".to_owned()),
        "main_resource_url".to_owned(),
    );
    assert_eq!(
        field_name("matchedCSSRules".to_owned()),
        "matched_css_rules".to_owned(),
    );
    assert_eq!(field_name("modernSSL".to_owned()), "modern_ssl".to_owned(),);
    assert_eq!(field_name("originURL".to_owned()), "origin_url".to_owned(),);
    assert_eq!(field_name("outerHTML".to_owned()), "outer_html".to_owned(),);
    assert_eq!(
        field_name("preferCSSPageSize".to_owned()),
        "prefer_css_page_size".to_owned(),
    );
    assert_eq!(
        field_name("remoteIPAddress".to_owned()),
        "remote_ip_address".to_owned(),
    );
    assert_eq!(
        field_name("removeDOMBreakpoint".to_owned()),
        "remove_dom_breakpoint".to_owned(),
    );
    assert_eq!(
        field_name("removeDOMStorageItem".to_owned()),
        "remove_dom_storage_item".to_owned(),
    );
    assert_eq!(field_name("reportAAA".to_owned()), "report_aaa".to_owned(),);
    assert_eq!(
        field_name("requestURL".to_owned()),
        "request_url".to_owned(),
    );
    assert_eq!(
        field_name("resourceIPAddressSpace".to_owned()),
        "resource_ip_address_space".to_owned(),
    );
    assert_eq!(field_name("scopeURL".to_owned()), "scope_url".to_owned(),);
    assert_eq!(field_name("scriptURL".to_owned()), "script_url".to_owned(),);
    assert_eq!(
        field_name("setDOMBreakpoint".to_owned()),
        "set_dom_breakpoint".to_owned(),
    );
    assert_eq!(
        field_name("setDOMStorageItem".to_owned()),
        "set_dom_storage_item".to_owned(),
    );
    assert_eq!(field_name("showCSS".to_owned()), "show_css".to_owned(),);
    assert_eq!(
        field_name("sourceMapURL".to_owned()),
        "source_map_url".to_owned(),
    );
    assert_eq!(field_name("sourceURL".to_owned()), "source_url".to_owned(),);
    assert_eq!(
        field_name("thresholdAA".to_owned()),
        "threshold_aa".to_owned(),
    );
    assert_eq!(
        field_name("thresholdAAA".to_owned()),
        "threshold_aaa".to_owned(),
    );
    assert_eq!(
        field_name("untrackIndexedDBForOrigin".to_owned()),
        "untrack_indexed_db_for_origin".to_owned(),
    );
    assert_eq!(
        field_name("untrackIndexedDBForStorageKey".to_owned()),
        "untrack_indexed_db_for_storage_key".to_owned(),
    );
    assert_eq!(
        field_name("userTypedURL".to_owned()),
        "user_typed_url".to_owned(),
    );
}
