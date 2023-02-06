use std::collections::HashMap;
use std::env;
use std::iter::Map;
///
/// Followed this tutorial for calling Web APIs:
/// https://rust-lang-nursery.github.io/rust-cookbook/web/clients/apis.html
///

use reqwest::{Client, Result};
use serde;

// ///
// /// api user login and id
// ///
// #[derive(serde::Serialize, serde::Deserialize, Debug)]
// struct User {
//     login: String,
//     id: u32,
// }



pub fn get_data(url: &str) {
}

///
/// Returns a HashMap of the github data.
///
/// # Arguments
///
/// * 'url' - the full github/npmjs url you want the data of
///
pub async fn get_github_data(url: &str) -> Result<String> {
    let owner = "NationalSecurityAgency";
    let repo = "ghidra";
    let request_url_str = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
                                    owner = owner,
                                    repo = repo);

    // println!("accessing url: {}", &request_url_str);

    let token = get_github_api_token();

    let client = Client::new();
    let res = client
                    .get(request_url_str)
                    .header("Authorization", token)
                    .header("User-Agent", "ECE461-repository-analyzer")
                    .send().await?;
    let status_code = res.status();
    println!("status code: {}", status_code);
    // println!("{}", res.text().await?);
    let mut js_text = res.text().await?;

    // formatting the string
    js_text.pop();
    js_text.remove(0);
    // this replace might not be needed
    js_text = str::replace(js_text.as_str(), r#"\"#, "/");

    struct JsonObject {
        values: HashMap<String, serde_json::Value>
    }
    let json_object = serde_json::from_str::<HashMap<String,serde_json::Value>>(&js_text).unwrap();

    println!("{:#?}", json_object.get("name"));
    Ok(status_code.to_string())
}


///
/// Returns the github link associated with the npmjs url
/// Returns "" if the link is not valid or there is no associated github link
///
/// # Arguments
///
/// * 'url' - The full npmjs url you want the associated github link of
///
pub async fn get_github_link(url: &str) -> Result<String> {
    let owner = "NationalSecurityAgency";
    let repo = "ghidra";
    let request_url_str = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
                                  owner = owner,
                                  repo = repo);

    // println!("accessing url: {}", &request_url_str);

    let token = get_github_api_token();

    let client = Client::new();
    let res = client
        .get(request_url_str)
        .header("Authorization", token)
        .header("User-Agent", "ECE461-repository-analyzer")
        .send().await?;
    let status_code = res.status();
    println!("status code: {}", status_code);


    Ok(status_code.to_string())

    // let mut s = String::new();
    // s.push_str("test");
    // return Result<String>::new(s);
}

//
// helper functions
//


fn get_github_api_token() -> String {
    let name = "GITHUB_TOKEN";
    let s: String;

    match env::var(name) {
        Ok(v) => { s = v; },
        Err(e) => { panic!("${} is not set ({})", name, e) }
    };
    return s;
}