///
/// Followed this tutorial for calling Web APIs:
/// https://rust-lang-nursery.github.io/rust-cookbook/web/clients/apis.html
///

// use std::collections::HashMap;
// use std::error::Error;
use reqwest::get;

///
/// api user login and id
///
#[derive(Debug)]
struct User {
    login: String,
    id: u32,
}



pub fn get_data(url: &str) {
}

///
/// Returns a HashMap of the github data.
///
/// # Arguments
///
/// * 'url' - the full github/npmjs url you want the data of
///
pub async fn get_github_data(url: &str) {
    //
    // let owner = "rust-lang-nursery";
    // let repo = "rust-cookbook";
    //
    // let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
    //                                 owner = owner,
    //                                 repo = repo);
    //
    // println!("{}", request_url);
    // let response = reqwest::get(&request_url).await?;
    //
    // let users: Vec<User> = response.json().await?;
    // println!("{:?}", users);
    // OK(())
}


///
/// Returns the github link associated with the npmjs url
/// Returns "" if the link is not valid or there is no associated github link
///
/// # Arguments
///
/// * 'url' - The full npmjs url you want the associated github link of
///
pub fn get_github_link(url: &str) -> String {
    let mut s = String::new();
    s.push_str("test");
    return s;
}

//
// helper functions
//
