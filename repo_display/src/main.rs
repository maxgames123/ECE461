mod repo_display;

fn main(){
    // Define several repositories
    let mut repo1 = repo_display::Repo{
        url: String::from("https://github.com/nullivex/nodist"),
        ramp_up: 0.5,
        correctness: 0.7,
        bus_factor: 0.3,
        responsive_maintainer: 0.4,
        license: 1,
        net_score : 0.9
    };
    repo1.ramp_up = 0.7;

    let mut repo2 = repo_display::Repo{
        url: String::from("https://github.com/abcd/Repo2.git"),
        ramp_up: 0.5,
        correctness: 0.75,
        bus_factor: 0.52,
        responsive_maintainer: 0.76,
        license: 1,
        net_score : 0.9
    };
    repo2.license = 1;

    // Display the repo info
    //repo_display::display_repo(&repo1);

    // Create the repo list
    let mut repo_list: Vec<repo_display::Repo> = Vec::new();
    repo_list.push(repo1);
    repo_list.push(repo2);

    // Display the info of the multiple repo
    repo_display::display_repo_list(&repo_list);
}

/*
The way the professor wants us to output the repos:

{"URL":"https://github.com/nullivex/nodist", "NET_SCORE":0.9, "RAMP_UP_SCORE":0.5, "CORRECTNESS_SCORE":0.7, "BUS_FACTOR_SCORE":0.3, "RESPONSIVE_MAINTAINER_SCORE":0.4, "LICENSE_SCORE":1}
{"URL":"https://github.com/nullivex/nodist", "NET_SCORE":0.9, "RAMP_UP_SCORE":0.7, "CORRECTNESS_SCORE":0.7, "BUS_FACTOR_SCORE":0.3, "RESPONSIVE_MAINTAINER_SCORE":0.4, "LICENSE_SCORE":1}
{"URL":"https://www.npmjs.com/package/browserify", "NET_SCORE":0.76, "RAMP_UP_SCORE":0.5, "CORRECTNESS_SCORE":0.7, "BUS_FACTOR_SCORE":0.3, "RESPONSIVE_MAINTAINER_SCORE":0.6, "LICENSE_SCORE":1}
{"URL":"https://github.com/cloudinary/cloudinary_npm", "NET_SCORE":0.6, "RAMP_UP_SCORE":0.5, "CORRECTNESS_SCORE":0.7, "BUS_FACTOR_SCORE":0.3, "RESPONSIVE_MAINTAINER_SCORE":0.2, "LICENSE_SCORE":1}
{"URL":"https://github.com/lodash/lodash", "NET_SCORE":0.5, "RAMP_UP_SCORE":0.5, "CORRECTNESS_SCORE":0.3, "BUS_FACTOR_SCORE":0.7, "RESPONSIVE_MAINTAINER_SCORE":0.6, "LICENSE_SCORE":1}
{"URL":"https://www.npmjs.com/package/express", "NET_SCORE":0, "RAMP_UP_SCORE":0.5, "CORRECTNESS_SCORE":0.7, "BUS_FACTOR_SCORE":0.3, "RESPONSIVE_MAINTAINER_SCORE":0.6, "LICENSE_SCORE":0}
*/