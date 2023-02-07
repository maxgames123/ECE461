use std::env;
use std::env::VarError;
use std::error::{Error};
use std::result::{Result};
use reqwest::{Client, Response, StatusCode};

///
/// Returns the github link associated with the npmjs package
///
/// # Arguments
///
/// * 'url' - The full npmjs url you want the associated github link of
///
pub async fn npmjs_get_github_link(url: &str) -> Result<String, Box<dyn Error>> {
    // docs of the api to call to get the github link
    // https://api-docs.npms.io/#api-Package

    let owner = "NationalSecurityAgency";
    let repo = "ghidra";
    let request_url_str = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
                                  owner = owner,
                                  repo = repo);

    // println!("accessing url: {}", &request_url_str);

    let token_res = get_github_api_token();
    if token_res.is_err() {
        return Err(Box::from(token_res.err().unwrap()))
    }
    let token = token_res.unwrap();

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


///
/// Returns the response body as a serde_json::Value
///
/// # Arguments
///
/// * 'url' - The full github url you want the associated request body from
///
pub async fn github_get_response_body(url: &str) -> Result<serde_json::Value, Box<dyn Error>> {
    let response_res = github_get_response(url).await;
    if response_res.is_err() {
        return Err(Box::try_from(response_res.err().unwrap()).unwrap());
    }
    let response = response_res.unwrap();
    let js_text_res = response.text().await;
    if js_text_res.is_err() {
        return Err(Box::try_from(js_text_res.err().unwrap()).unwrap())
    }
    let js_text = js_text_res.unwrap().to_owned();
    let js_obj = serde_json::from_str(&js_text);
    if js_obj.is_err() {
        return Err(Box::from(js_obj.err().unwrap()))
    }
    Ok(js_obj.unwrap())
}


///
/// Returns the StatusCode of a request to a Github url
///
/// # Arguments
///
/// * 'url' - The full github url you want the StatusCode from
///
pub async fn github_get_status(url: &str) -> Result<StatusCode, Box<dyn Error>> {
    let response_res = github_get_response(url).await;
    if response_res.is_err() {
        return Err(Box::try_from(response_res.err().unwrap()).unwrap());
    }
    Ok(response_res.unwrap().status().to_owned())
}


///
/// Returns the Response object from a github url
///
/// # Arguments
///
/// * 'url' - The full github url you want the response from
///
pub async fn github_get_response(url: &str) -> Result<Response, Box<dyn Error>> {
    let token_res = get_github_api_token();
    if token_res.is_err() {
        return Err(Box::try_from(token_res.err().unwrap()).unwrap());
    }
    let token = token_res.unwrap();

    let client = Client::new();
    let response_res = client
        .get(url)
        .header("Authorization", token)
        .header("User-Agent", "ECE461-repository-analyzer")
        .send().await;
    if response_res.is_err() {
        return Err(Box::try_from(response_res.err().unwrap()).unwrap());
    }
    Ok(response_res.unwrap())
}


#[deprecated]
///
/// Returns a HashMap of the github data.
///
/// # Arguments
///
/// * 'url' - the page that you want the body of
///
pub async fn get_github_data(url: &str) -> Result<serde_json::Value, Box<dyn Error>> {

    // println!("accessing url: {}", &request_url_str);

    let token_res = get_github_api_token();
    if token_res.is_err() {
        return Err(Box::try_from(token_res.err().unwrap()).unwrap());
    }
    let token = token_res.unwrap();

    let client = Client::new();
    let response_res = client
        .get(url)
        .header("Authorization", token)
        .header("User-Agent", "ECE461-repository-analyzer")
        // .header("Accept", "application/xml")
        .send().await;

    if response_res.is_err() {
        return Err(Box::try_from(response_res.err().unwrap()).unwrap());
    }
    let response = response_res.unwrap();

    // let status_code = response.status();
    // println!("status code: {}", status_code);
    let js_text = response.text().await?.to_owned();

    let json_object: serde_json::Value = serde_json::from_str(&js_text).unwrap();

    Ok(json_object)
}


//
// helper functions
//


fn get_github_api_token() -> Result<String, VarError> {
    let name = "GITHUB_TOKEN";
    let res = env::var(name);
    if res.is_err() {
        println!("${} is not set in Enviromental Variables", name);
        return Err(res.err().unwrap())
    }
    Ok(res.unwrap())
}