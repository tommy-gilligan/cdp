use codegen::Struct;
use codegen::Scope;

pub fn r#gen(domain: crate::parser::Domain) -> Scope {
    let mut scope = Scope::new();
    let module = scope.new_module(&domain.domain);
    module.vis("pub");

    if let Some(ref types) = domain.types {
        for t in types {
            match t.r#type.as_str() {
                "object" => {
                    let mut s = module.new_struct(&t.id);

                    if let Some(e) = &t.r#properties {
                        for v in e {
                            if let Some(r) = &v.r#ref {
                                let c = match v.optional {
                                    Some(true) => {
                                        codegen::Type::new(format!("Option<{}>", r))
                                    },
                                    _ => {
                                        codegen::Type::new(r)
                                    }
                                };

                                let g = s.new_field(v.name.clone(), c);
                                if let Some(description) = &v.description {
                                    g.doc(&*description);
                                };
                            } else {
                                let c = match v.optional {
                                    Some(true) => {
                                        codegen::Type::new("Option<String>")
                                    },
                                    _ => {
                                        codegen::Type::new("String")
                                    }
                                };

                                let g = s.new_field(v.name.clone(), c);
                                if let Some(description) = &v.description {
                                    g.doc(&*description);
                                };
                            }
                        }
                    }

                    s.vis("pub");
                    if let Some(description) = &t.description {
                        s.doc(&description);
                    };
                },
                "string" => {
                    if let Some(e) = &t.r#enum {
                        let mut s = module.new_enum(&t.id);
                        for v in e {
                            s.new_variant(v);
                        }

                        s.vis("pub");
                        if let Some(description) = &t.description {
                            s.doc(&description);
                        };
                    } else {
                    }
                }
                "boolean" => {
                }
                "array" => {
                }
                _ =>  {}
            }
        }
    }

    for t in domain.commands {
        let g = module.new_fn(&t.name).vis("pub");

        if let Some(description) = &t.description {
            g.doc(&*description);
        };

        if let Some(parameters) = &t.parameters {
            for p in parameters {
                let l = match p.r#type.clone() {
                    None if p.r#ref.is_some() => Some(p.r#ref.clone().unwrap()),
                    Some(s) if p.r#type.clone() == Some("string".to_owned()) => Some("String".to_owned()),
                    _ => None
                };

                if let Some(r) = l {
                    let c = match p.optional {
                        Some(true) => {
                            codegen::Type::new(format!("Option<{}>", r))
                        },
                        _ => {
                            codegen::Type::new(r)
                        }
                    };

                    g.arg(&p.name, c);
                }
            }
        }
    }
    scope
}
