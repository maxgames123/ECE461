// It represents a repository
pub struct Repo{
    pub url: String,
    pub net_score: f32,
    pub ramp_up: f32,
    pub correctness: f32,
    pub bus_factor: f32,
    pub responsive_maintainer: f32,
    pub license: i32,
}

pub fn display_repo(repo: &Repo){
    println!("{{\"URL\":\"{}\", \"NET_SCORE\":{}, \"RAMP_UP_SCORE\":{}, \"CORRECTNESS_SCORE\":{}, \"BUS_FACTOR_SCORE\":{}, \"RESPONSIVE_MAINTAINER_SCORE\":{}, \"LICENSE_SCORE\":{}}}", repo.url, repo.net_score, repo.ramp_up, repo.correctness, repo.bus_factor, repo.responsive_maintainer, repo.license);
}

pub fn display_repo_list(repo_list: &Vec<Repo>){
    for (_i, repo) in repo_list.iter().enumerate(){
        display_repo(repo);
    }
}