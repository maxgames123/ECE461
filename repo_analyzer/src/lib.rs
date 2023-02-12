extern crate libc;
extern crate core;

mod repo_list;
mod url_input;
mod metric_calculations;
mod rest_api;
mod read_url_file;

use std::error::Error;
use libc::c_char;
use std::ffi::{CString, CStr};

// Followed instructions from:
//  creating library:
//  https://developers.redhat.com/blog/2017/11/16/speed-python-using-rust
//  setting up library for python usage:
//  https://docs.rs/cpython/latest/cpython/
//


// Example function on converting a C string to and from a rust string
// #[no_mangle]
// pub extern fn get_string(input: *const c_char) -> *mut c_char {
//     let input_str = unsafe {
//         assert!(!input.is_null());
//         CStr::from_ptr(input).to_str().unwrap()
//     };
//     let output_str = format!("{} rust", input_str);
//     let output = CString::new(output_str).unwrap();
//     output.into_raw()
// }


#[no_mangle]
pub async extern "C" fn rust_start_point(input: *const c_char) {

    let filename = unsafe {
        assert!(!input.is_null());
        CStr::from_ptr(input).to_str().unwrap()
    };  

    let url_list = read_url_file::read_lines(filename); // returns urls as a list of strings
    let mut repos = repo_list::RepoList::new(); // creates a RepoList object

    for repo_url in url_list { // creates a Repo object for each url and adds it to RepoList
        let (domain, data) = url_input::get_data(&repo_url);
        let owner = &data[0];
        let package = &data[1];

        if !domain.eq("npmjs") && !domain.eq("github"){
            println!("Domain must either be npmjs or github!\n");
            continue;
        }

        if domain.eq("npmjs") {
            let github_link = rest_api::npmjs_get_repository_link(&data[0], &data[1]).await;
            repos.add_repo(repo_list::Repo {url: github_link.unwrap(), ..Default::default()});
        } else {
            repos.add_repo(repo_list::Repo {url: repo_url, ..Default::default()});
        }

    
        // What we should do here:
        // 1) Get data from GitHub for each metric
        // 2) Calculate each metric
        // 3) Update line below with the scores. It just gives default values for now.
    }
    
    repos.sort_by_net_score(); // will sort the RepoList by trustworthiness.
    repos.display(); // will print RepoList to stdout in the desired format!
}


// main function for testing stuff
// run test_web_api().await to run different examples of using the rest_api functions.
// Make sure to set your github token in your environmental variables under the name 'GITHUB_TOKEN'
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    test_web_api().await;
    Ok(())
}




async fn test_web_api() -> Result<(), Box<dyn Error>> {
    get_github_metrics().await;
    // get_response().await;
    // get_status_code().await;
    // get_body().await;

    Ok(())
}

async fn get_github_metrics() -> Result<(), Box<dyn Error>> {
    let v = rest_api::github_get_metrics(&tutorial_owner(), &tutorial_repo()).await;
    // let v = rest_api::npmjs_get_repository_link("browserify").await;
    println!("Github metrics:");
    println!("{:#?}", v);
    Ok(())
}

async fn get_response() -> Result<(), Box<dyn Error>> {
    let v = rest_api::github_get_response(&tutorial_owner(), &tutorial_repo(), None).await?;
    println!("Response:");
    println!("{:#?}", v);
    Ok(())
}

async fn get_status_code() -> Result<(), Box<dyn Error>> {
    let v = rest_api::github_get_status(&tutorial_owner(), &tutorial_repo()).await;
    println!("Status code:");
    println!("{}", v.unwrap().as_u16());
    Ok(())
}

async fn get_body() -> Result<(), Box<dyn Error>> {

    let v = rest_api::github_get_response_body(&tutorial_owner(), &tutorial_repo(), None).await;
    let body = v.unwrap();
    println!("Response Body:");
    println!("{:#?}", body);
    println!("accessing values inside the body:");
    println!("{:#?}", body[0]);
    println!("{}", body[0]["id"]);
    Ok(())
}

fn tutorial_owner() -> String {
    return "NationalSecurityAgency".to_owned();
}

fn tutorial_repo() -> String {
    return "ghidra".to_owned();
    // rest_api::get_github_data(url);
    // url_input::run(url);
}