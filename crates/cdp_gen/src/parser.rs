use serde::Deserialize;
use serde_json::Result;

#[derive(Deserialize, Debug)]
pub struct Event {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Option<Vec<Parameter>>
}

#[derive(Deserialize, Debug)]
pub struct TypeType {
    pub name: String,
    pub r#type: Option<String>,
    #[serde(rename = "$ref")]
    pub r#ref: Option<String>,
    pub description: Option<String>,
    pub optional: Option<bool>
}

#[derive(Deserialize, Debug)]
pub struct Type {
    pub id: String,
    pub description: Option<String>,
    pub r#type: String,
    // these are co-incident with type
    pub r#enum: Option<Vec<String>>,
    pub r#properties: Option<Vec<TypeType>>,
}

#[derive(Deserialize, Debug)]
pub struct Parameter {
    pub name: String,
    pub description: Option<String>,
    pub optional: Option<bool>,
    #[serde(rename = "$ref")]
    pub r#ref: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Return {
    pub name: String,
    pub description: Option<String>,
    pub r#type: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct Command {
    pub name: String,
    pub description: Option<String>,
    pub experimental: Option<bool>,
    // please can i not need an option around a vec that could just be empty?
    pub parameters: Option<Vec<Parameter>>,
    pub returns: Option<Vec<Return>>
}

#[derive(Deserialize, Debug)]
pub struct Domain {
    pub domain: String,
    pub types: Option<Vec<Type>>,
    pub commands: Vec<Command>,
    pub events: Option<Vec<Event>>,
    pub dependencies: Option<Vec<String>>,
    pub experimental: Option<bool>,
    pub description: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct Version {
    pub major: String,
    pub minor: String,
}

#[derive(Deserialize, Debug)]
pub struct Main {
    pub version: Version,
    pub domains: Vec<Domain>
}
