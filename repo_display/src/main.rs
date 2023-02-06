mod repo_display;

fn main(){
    // Define several repositories
    let mut repo1 = repo_display::Repo{
        name: String::from("Repo1"),
        url: String::from("https://github.com/abcd/repo1.git"),
        ramp_up: 0.0,
        correctness: 0.8,
        bus_factor: 0.5,
        responsive_maintainer: 0.9,
        license: true
    };
    repo1.ramp_up = 0.7;

    let mut repo2 = repo_display::Repo{
        name: String::from("Repo2"),
        url: String::from("https://github.com/abcd/Repo2.git"),
        ramp_up: 0.5,
        correctness: 0.75,
        bus_factor: 0.52,
        responsive_maintainer: 0.76,
        license: true
    };
    repo2.license = false;

    // Display the repo info
    println!("========== Display a repo ==========");
    repo_display::display_repo(&repo1);

    // Create the repo list
    let mut repo_list: Vec<repo_display::Repo> = Vec::new();
    repo_list.push(repo1);
    repo_list.push(repo2);

    // Display the info of the multiple repo
    println!("\n========== Display a repo list ===========");
    repo_display::display_repo_list(&repo_list);
}
