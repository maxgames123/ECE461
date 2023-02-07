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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    testt().await;
    Ok(())
}

async fn testt() -> Result<String, Box<dyn Error>> {
    let owner = "NationalSecurityAgency";
    let repo = "ghidra";
    let url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
                                  owner = owner,
                                  repo = repo).to_owned();
    let v = rest_api::github_get_response_body(&url).await?;
    println!("{:#?}", v);
    Ok("ok".to_string())
}