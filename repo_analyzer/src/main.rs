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
use std::process::Command;

// run test_web_api().await to run different examples of using the rest_api functions.
// Make sure to set your github token in your environmental variables under the name 'GITHUB_TOKEN'
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // test_web_api().await;
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 2 {
        println!("Incorrect number of arguments!");
        return Ok(());
    } else if args.len() == 1 {
        run_help();
        return Ok(());
    }

    if args[1] == "install"{
        run_install();
    } else if args[1] == "build"{
        run_build();
    } else if args[1] == "test"{
        run_test();
    } else {
        run_url(&args[1]).await;
    }

    Ok(())
}

fn run_install() {
    Command::new("cargo")
        .args(&["install", "cargo-edit"])
        .output()
        .expect("failed to execute process");

    Command::new("cargo")
        .args(&["add", "reqwest"])
        .output()
        .expect("failed to execute process");

    Command::new("cargo")
        .args(&["add", "serde"])
        .output()
        .expect("failed to execute process");

    Command::new("cargo")
        .args(&["add", "serde_json"])
        .output()
        .expect("failed to execute process");

    Command::new("cargo")
        .args(&["add", "tokio"])
        .output()
        .expect("failed to execute process");
    
    Command::new("cargo")
        .args(&["add", "substring"])
        .output()
        .expect("failed to execute process");

    Command::new("cargo")
        .args(&["add", "base64"])
        .output()
        .expect("failed to execute process");

        Command::new("cargo")
        .args(&["add", "async-recursion"])
        .output()
        .expect("failed to execute process");

    Command::new("cargo")
        .args(&["add", "regex"])
        .output()
        .expect("failed to execute process");
    
    Command::new("cargo")
        .args(&["add", "log"])
        .output()
        .expect("failed to execute process");

    Command::new("cargo")
        .args(&["add", "env_logger"])
        .output()
        .expect("failed to execute process");

        println!("11 dependencies installed...");
}

fn run_build() {
    Command::new("cargo")
        .arg("build")
        .output()
        .expect("failed to execute process");
}

fn run_test() {
    // Implement the run_test function here
}

async fn run_url(filename: &str) {
    // Implement the run_url function here

    let url_list = read_url_file::read_lines(filename); // returns urls as a list of strings
    let mut repos = repo_list::RepoList::new(); // creates a RepoList object

    for repo_url in url_list { // creates a Repo object for each url and adds it to RepoList
        let (domain, data) = url_input::get_data(&repo_url);
        let mut owner = data[0].as_str();
        let mut package = data[1].as_str();

        if !domain.eq("npmjs") && !domain.eq("github"){
            println!("Domain must either be npmjs or github!\n");
            continue;
        }

        if domain.eq("npmjs") {
            let github_link = match rest_api::npmjs_get_repository_link(owner, package).await {
                Ok(github_link) => github_link,
                Err(e) => "".to_owned()
            };

            if github_link.eq("") {
                // The function failed to get the github link so it returns ""
                // adds an entry
                repos.add_repo(repo_list::Repo {url: repo_url, ..Default::default()});
                continue;
            }

            let (git_domain, git_data) = url_input::get_data(&github_link);
            owner = git_data[0].as_str();
            package = git_data[1].as_str();

            let codebase_length = match rest_api::github_get_codebase_length(owner , package).await {
                Ok(codebase_length) => codebase_length,
                Err(e) => "0.0".to_owned()
            };
    
            println!("code len: {}", codebase_length);
    
            let opened_issues = match rest_api::github_get_open_issues(owner , package).await {
                Ok(opened_issues) => opened_issues,
                Err(e) => "0.0".to_owned()
            };
    
            println!("open issues: {}", opened_issues);
    
            let license = match rest_api::github_get_license(owner , package).await {
                Ok(license) => license,
                Err(e) => "0.0".to_owned()
            };
    
            println!("license: {}", license);
    
            let number_of_forks = match rest_api::github_get_number_of_forks(owner , package).await {
                Ok(number_of_forks) => number_of_forks,
                Err(e) => "0.0".to_owned()
            };
    
            println!("number_of_forks: {}", number_of_forks);

            let ru = metric_calculations::get_ramp_up_time(&codebase_length);
            let c = metric_calculations::get_correctness(&opened_issues);
            let bf = metric_calculations::get_bus_factor(&number_of_forks);
            let l = metric_calculations::get_license(&license);
            let rm = metric_calculations::get_responsive_maintainer();

            let metrics = [ru, c, bf, l]; // responsive maintainer is omitted
            let o = metric_calculations::get_overall(&metrics);

            repos.add_repo(repo_list::Repo {url : repo_url, net_score : o, ramp_up : ru, correctness : c, bus_factor : bf, responsive_maintainer : rm, license : l});
            continue;

        }
        
        // SHOULD WE SET THE GITHUB_TOKEN ENVIRONMENT VAR IN THE PROGRAM?

        let codebase_length = match rest_api::github_get_codebase_length(owner , package).await {
            Ok(codebase_length) => codebase_length,
            Err(e) => "0.0".to_owned()
        };

        println!("code len: {}", codebase_length);

        let opened_issues = match rest_api::github_get_open_issues(owner , package).await {
            Ok(opened_issues) => opened_issues,
            Err(e) => "0.0".to_owned()
        };

        println!("open issues: {}", opened_issues);

        let license = match rest_api::github_get_license(owner , package).await {
            Ok(license) => license,
            Err(e) => "0.0".to_owned()
        };

        println!("license: {}", license);

        let number_of_forks = match rest_api::github_get_number_of_forks(owner , package).await {
            Ok(number_of_forks) => number_of_forks,
            Err(e) => "0.0".to_owned()
        };

        println!("number_of_forks: {}", number_of_forks);

        let ru = metric_calculations::get_ramp_up_time(&codebase_length);
        let c = metric_calculations::get_correctness(&opened_issues);
        let bf = metric_calculations::get_bus_factor(&number_of_forks);
        let l = metric_calculations::get_license(&license);
        let rm = metric_calculations::get_responsive_maintainer();

        let metrics = [ru, c, bf, l]; // responsive maintainer is omitted
        let o = metric_calculations::get_overall(&metrics);

        repos.add_repo(repo_list::Repo {url : repo_url, net_score : o, ramp_up : ru, correctness : c, bus_factor : bf, responsive_maintainer : rm, license : l});
    }

    repos.sort_by_net_score(); // will sort the RepoList by trustworthiness.
    repos.display(); // will print RepoList to stdout in the desired format.
}

fn run_help() {
    // Implement the run_help function here
}
