mod repo_list;

fn main(){
    repo_list::run();
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