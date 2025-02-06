#![allow(unused)]
#![allow(dead_code)]
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Event {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Option<Vec<Parameter>>,
    pub experimental: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Items {
    #[serde(rename = "$ref")]
    pub r#ref: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Type {
    pub id: String,
    pub description: Option<String>,
    pub r#type: String,
    // these are co-incident with type
    pub r#enum: Option<Vec<String>>,
    pub r#properties: Option<Vec<Parameter>>,
    pub r#items: Option<Items>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub description: Option<String>,
    pub optional: Option<bool>,
    pub experimental: Option<bool>,
    #[serde(rename = "$ref")]
    pub r#ref: Option<String>,
    pub r#type: Option<String>,
    pub r#items: Option<Items>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Command {
    pub name: String,
    pub description: Option<String>,
    pub experimental: Option<bool>,
    // please can i not need an option around a vec that could just be empty?
    pub parameters: Option<Vec<Parameter>>,
    pub returns: Option<Vec<Parameter>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Domain {
    pub domain: String,
    pub types: Option<Vec<Type>>,
    pub commands: Vec<Command>,
    pub events: Option<Vec<Event>>,
    pub dependencies: Option<Vec<String>>,
    pub experimental: Option<bool>,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Version {
    pub major: String,
    pub minor: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Main {
    pub version: Version,
    pub domains: Vec<Domain>,
}
