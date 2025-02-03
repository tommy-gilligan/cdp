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

use regex::{Regex, Captures};

fn process_doc(s: String) -> String {
    let re = Regex::new(r"(\s)(<[^>]+>)").unwrap();
    let a = re.replace_all(&s, |caps: &Captures| {
        format!("{}`{}`", &caps[1], &caps[2])
    }).into_owned();

    // let re = Regex::new(r"(https?://\S+)").unwrap();
    // let b = re.replace_all(&a, |caps: &Captures| {
    //     format!("<{}>", &caps[1])
    // }).into_owned();

    let re = Regex::new(r"(\[|\])").unwrap();
    re.replace_all(&a, |caps: &Captures| {
        format!("\\{}", &caps[1])
    }).into_owned()
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
                        "string" => Some("String".to_owned()),
                        "integer" => Some("u64".to_owned()),
                        "number" => Some("f64".to_owned()),
                        "boolean" => Some("bool".to_owned()),

                        "object" => Some("()".to_owned()),
                        "any" => Some("()".to_owned()),
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
        Some(_s) if p.r#type.clone() == Some("object".to_owned()) => Some("()".to_owned()),
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
    match name.clone().as_str() {
        "DOM" => "dom".to_owned(),
        "DOMDebugger" => "dom_debugger".to_owned(),
        "DOMSnapshot" => "dom_snapshot".to_owned(),
        "DOMStorage" => "dom_storage".to_owned(),
        "IO" => "io".to_owned(),
        "PWA" => "pwa".to_owned(),
        "IndexedDB" => "indexed_db".to_owned(),
        "CSS" => "css".to_owned(),
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
    }
}

pub fn main_client(domains: &[crate::parser::Domain]) -> Scope {
    let mut scope = Scope::new();
    let client_impl = scope.new_impl("crate::Client");

    for domain in domains {
        let function = client_impl.new_fn(field_name(domain.domain.to_owned()));
        function.vis("pub");
        function.arg_mut_self();

        let domain_client = format!("{}::Client", field_name(domain.domain.to_owned()));
        function.ret(&domain_client);

        function.line(format!("{}(self)", &domain_client));

        if let Some(true) = domain.experimental {
            function.attr("cfg(feature = \"experimental\")");
        }
    }

    scope
}

pub fn modules(domains: &[crate::parser::Domain]) -> String {
    let mut s = String::new();
    for domain in domains {
        if let Some(true) = domain.experimental {
            s.push_str("#[cfg(feature = \"experimental\")]\n");
        }
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
                    let s = module.new_struct(variant_name(t.id.clone()));
                    s.derive("Debug");
                    s.derive("PartialEq");
                    s.derive("crate::Deserialize");
                    s.derive("crate::Serialize");
                    s.attr("serde(deny_unknown_fields)");

                    if let Some(e) = &t.r#properties {
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
                    }

                    s.vis("pub");
                    if let Some(description) = &t.description {
                        s.doc(process_doc(description.clone()));
                    };
                }
                "string" => {
                    if let Some(e) = &t.r#enum {
                        let s = module.new_enum(&t.id);
                        s.derive("Debug");
                        s.derive("PartialEq");
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
                                "string" => Some("String".to_owned()),
                                "integer" => Some("u64".to_owned()),
                                "number" => Some("f64".to_owned()),
                                "any" => Some("()".to_owned()),
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
        g.attr("serde(deny_unknown_fields)");
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
            r.attr("serde(deny_unknown_fields)");
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
            r.attr("serde(deny_unknown_fields)");
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
    client.vis("pub");
    client.generic("'a");
    client.tuple_field("pub &'a mut crate::Client");

    let client_impl = module.new_impl("Client");
    client_impl.target_generic("'_");
    for t in &domain.commands {
        let function = client_impl.new_fn(field_name(escape_field_name(t.name.clone())));
        function.vis("pub");
        function.arg_mut_self();
        function.set_async(true);
        function.ret(format!("{}Return", variant_name(t.name.clone())));

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
            "self.0.send_command(\"{}.{}\", request).await",
            domain.domain.to_owned(),
            t.name.clone()
        ));
    }

    module
}
