// It represents a repository
pub struct Repo{
    pub name: String,
    pub url: String,
    pub ramp_up: f32,
    pub correctness: f32,
    pub bus_factor: f32,
    pub responsive_maintainer: f32,
    pub license: bool
}

pub fn display_repo(repo: &Repo){
    println!("Repository name: {}", repo.name);
    println!("Repository url: {}", repo.url);
    println!("RampUp: {}", repo.ramp_up);
    println!("Correctness: {}", repo.correctness);
    println!("BusFactor: {}", repo.bus_factor);
    println!("ResponsiveMaintainer: {}", repo.responsive_maintainer);
    println!("License: {}", repo.license);
}

pub fn display_repo_list(repo_list: &Vec<Repo>){
    for (i, repo) in repo_list.iter().enumerate(){
        println!("index: {}", i + 1);
        display_repo(repo);
        println!("");
    }
}
