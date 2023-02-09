use std::fs::File;
use std::io::{ self, BufRead, BufReader };
use std::vec::Vec;
use std::default::Default;

// It represents a repository
pub struct Repo{
    pub url: String,
    pub net_score: f32,
    pub ramp_up: f32,
    pub correctness: f32,
    pub bus_factor: f32,
    pub responsive_maintainer: f32,
    pub license: f32,
}

impl Default for Repo {
    fn default() -> Self {
        Repo {
            url: String::new(),
            net_score: 0.0,
            ramp_up: 0.0,
            correctness: 0.0,
            bus_factor: 0.0,
            responsive_maintainer: 0.0,
            license: 0.0,
        }
    }
}

// A list of repositories
pub struct RepoList {
    pub repos: Vec<Repo>,
}

impl RepoList {
    pub fn new() -> RepoList {
        RepoList { repos: Vec::new() }
    }

    pub fn add_repo(&mut self, repo: Repo) {
        self.repos.push(repo);
    }

    pub fn sort_by_net_score(&mut self) {
        self.repos.sort_by(|a, b| b.net_score.partial_cmp(&a.net_score).unwrap());
    }

    pub fn display(&self) {
        for repo in &self.repos {
            println!("{{\"URL\":\"{}\", \"NET_SCORE\":{}, \"RAMP_UP_SCORE\":{}, \"CORRECTNESS_SCORE\":{}, \"BUS_FACTOR_SCORE\":{}, \"RESPONSIVE_MAINTAINER_SCORE\":{}, \"LICENSE_SCORE\":{}}}", 
                repo.url, repo.net_score, repo.ramp_up, repo.correctness, repo.bus_factor, repo.responsive_maintainer, repo.license);
        }
    }
}


pub fn run() {
    // Read in the url_list and then create Repo instances
    println!("Hello from repo_list.rs in the Rust library!");
    let path: String = "test_url_file/URL".to_string();
    let mut repo_list: Vec<Repo> = Vec::new();
    get_repo_list(path, &mut repo_list);

    // Print the repo list
    display_repo_list(&repo_list);
}

pub fn display_repo(repo: &Repo){
    println!("{{\"URL\":\"{}\", \"NET_SCORE\":{}, \"RAMP_UP_SCORE\":{}, \"CORRECTNESS_SCORE\":{}, \"BUS_FACTOR_SCORE\":{}, \"RESPONSIVE_MAINTAINER_SCORE\":{}, \"LICENSE_SCORE\":{}}}", 
        repo.url, repo.net_score, repo.ramp_up, repo.correctness, repo.bus_factor, repo.responsive_maintainer, repo.license);
}

pub fn display_repo_list(repo_list: &Vec<Repo>){
    for (_i, repo) in repo_list.iter().enumerate(){
        display_repo(repo);
    }
}

fn read_file_lines(filename: String) -> io::Lines<BufReader<File>> {
    // Open the file
    let file = File::open(filename).unwrap(); 
    // Return an iterator of the lines of the file.
    return io::BufReader::new(file).lines(); 
}

fn get_repo_list(path: String, repo_list: &mut Vec<Repo>){
    let lines = read_file_lines(path);
    // Iterate over the lines of the file
    for line in lines {
        // Create a Repo instance and then push it to the vector
        let url = line.unwrap();
        let mut repo = Repo{
            url: String::from(url),
            net_score : 0.0,
            ramp_up: 0.0,
            correctness: 0.0,
            bus_factor: 0.0,
            responsive_maintainer: 0.0,
            license: 0.0
        };
        repo.license = 1.0;
        repo_list.push(repo);
    }
}