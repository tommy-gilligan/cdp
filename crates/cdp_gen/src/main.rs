mod parser;
use parser::*;

mod r#gen;
use r#gen::*;

// when it comes to type identifiers, do whatever is minimum to appease default clippy
#[tokio::main]
async fn main() {
    // /Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --remote-debugging-port=9210
    // let mut resp: Main = reqwest::get("http://localhost:9210/json/protocol")
    //     .await.unwrap()
    //     .json()
    //     .await.unwrap();

    let data: Vec<u8> = std::fs::read("a.json").unwrap();
    let mut resp: Main = serde_json::from_str(&String::from_utf8(data).unwrap()).unwrap();

    let schema = resp.domains.pop().unwrap();
    let schema = resp.domains.pop().unwrap();
    let scope = r#gen(schema);
    println!("{}", scope.to_string());
}
