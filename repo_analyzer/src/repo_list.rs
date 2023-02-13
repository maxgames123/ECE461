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