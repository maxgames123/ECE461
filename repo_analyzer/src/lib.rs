extern crate libc;
mod repo_list;
mod url_input;
mod metric_calculations;
mod rest_api;

use std::error::Error;
use libc::c_char;
use std::ffi::{CString, CStr};

// Followed instructions from:
//  creating library:
//  https://developers.redhat.com/blog/2017/11/16/speed-python-using-rust
//  setting up library for python usage:
//  https://docs.rs/cpython/latest/cpython/
//

#[no_mangle]
pub extern fn c_entry() {
    println!("Hello World from Rust!");

}

#[no_mangle]
pub extern fn get_string(input: *const c_char) -> *mut c_char {
    let input_str = unsafe {
        assert!(!input.is_null());
        CStr::from_ptr(input).to_str().unwrap()
    };
    let output_str = format!("{} rust", input_str);
    let output = CString::new(output_str).unwrap();
    output.into_raw()
}


#[no_mangle]
pub extern "C" fn display_repo_list() {
    repo_list::run();
}


#[no_mangle]
pub async extern "C" fn print_score_from_url(input: *const c_char) {
    let url = unsafe {
        assert!(!input.is_null());
        CStr::from_ptr(input).to_str().unwrap()
    };

    url_input::run(url);
}


// main function for testing stuff
// run test_web_api().await to print out the response for the given url.
// Make sure to set your github token in your environmental variables under the name 'GITHUB_TOKEN'
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    test_web_api().await;
    Ok(())
}


async fn test_web_api() -> Result<(), Box<dyn Error>> {
    get_response().await;
    get_status_code().await;
    Ok(())
}

async fn get_response() -> Result<(), Box<dyn Error>> {
    let url = tutorial_url();
    let v = rest_api::github_get_response_body(&url).await?;
    println!("Response:");
    println!("{:#?}", v);
    Ok(())
}

async fn get_status_code() -> Result<(), Box<dyn Error>> {
    let url = tutorial_url();
    let v = rest_api::github_get_status(&url).await;
    println!("Status code:");
    println!("{}", v.unwrap().as_u16());
    Ok(())
}

async fn get_body() -> Result<(), Box<dyn Error>> {
    let url = tutorial_url();
    let v = rest_api::github_get_response_body(&url).await;
    let body = v.unwrap();
    println!("Response Body:");
    println!("{:#?}", body);
    println!("accessing values inside the body:");
    println!("{:#?}", body[0]);
    println!("{}", body[0]["id"]);
    Ok(())
}

fn tutorial_url() -> String {
    let owner = "NationalSecurityAgency";
    let repo = "ghidra";
    format!("https://api.github.com/repos/{}/{}/stargazers", owner, repo)
}