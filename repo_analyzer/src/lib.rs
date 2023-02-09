extern crate libc;
mod repo_list;
mod url_input;
mod metric_calculations;
mod rest_api;
mod read_url_file;

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
pub extern "C" fn print_score_from_url(input: *const c_char) {
    let url = unsafe {
        assert!(!input.is_null());
        CStr::from_ptr(input).to_str().unwrap()
    };

    // rest_api::get_github_data(url);
    url_input::run(url);
}

#[no_mangle]
pub extern "C" fn rust_start_point(input: *const c_char) {
    let filename = unsafe {
        assert!(!input.is_null());
        CStr::from_ptr(input).to_str().unwrap()
    };  

    let url_list = read_url_file::read_lines(filename); // returns urls as a list of strings

    let mut repos = repo_list::RepoList::new(); // creates a RepoList object

    for repo_url in url_list { // creates a Repo object for each url and adds it to RepoList
        // What we should do here:
        // 1) Get data from GitHub for each metric
        // 2) Calculate each metric
        // 3) Update line below with the scores. It just gives default values for now.
        repos.add_repo(repo_list::Repo {url: repo_url, ..Default::default()});
    }
    
    repos.sort_by_net_score(); // will sort the RepoList by trustworthiness.
    repos.display(); // will print RepoList to stdout in the desired format!
}