use reqwest::Client;
use std::error::Error;
use regex:: Regex;


async fn helper() -> String {
    let url = "https://www.npmjs.com/package/browserify";
    let data = get_data(url);
    // domain is either going to be npmjs or github or none of the above
    let domain = &data[0];
    if domain == "npmjs" {
        let package_name = &data[1];
        let github_link_result = npmjs_get_github_link(package_name).await;
        if let Ok(github_link) = github_link_result {
            println!("GitHub link: {}", github_link);
            return github_link;
        } else {
            let error = github_link_result.unwrap_err();
            println!("Error: {}", error);
            std::process::exit(1);
        }
    } else if domain == "github" {
        return url.to_string();

    } else {
        eprintln!("Not a valid npmjs URL: {}", url);
        std::process::exit(1);
    }
}



// Retrieves the GitHub link for a npmjs package
pub async fn npmjs_get_github_link(package_name: &str) -> Result<String, Box<dyn Error>> {    
    let client = Client::new();
    let npms_api_url = format!("https://api.npms.io/v2/package/{}", package_name);
    // Example: npms_api_url = https://api.npms.io/v2/package/express
    
    // Send a GET request to the NPMS API URL
    let res = client
        .get(npms_api_url)
        .send().await?;
        
    /*
     "repository": {
                "type": "git",
                "url": "git+https://github.com/expressjs/express.git"
            },
    */

    let js_text = res.text().await?.to_owned();
    let json_object: serde_json::Value = serde_json::from_str(&js_text).unwrap();
    // Retrieves the Github URL in the API's return json object "repository" field
    let repository_url = json_object["repository"]["url"].as_str();
    let github_url = repository_url.unwrap();

    if github_url.contains("github.com/") {
        // Use a regular expression to extract the GitHub link from the repository URL
        // INPUT: "git+https://github.com/expressjs/express.git"
        // RETURN: https://github.com/expressjs/express
        let re = Regex::new(r"(https://github\.com/[^.]*)").unwrap();
        let github_link = re.captures(&github_url).unwrap(); 
        Ok(format!("https://github.com/{}", github_link[1].to_string()))
    } else {
        let error_message = "The repository URL doesn't contain 'github.com/'";
        Err(Box::from(error_message))
    }
}


// Returns true if there was a match to the regex
pub fn is_valid(url: &str, regex: &Regex) -> bool {
    regex.is_match(url)
}

/*
Input: "https://www.npmjs.com/package/request"
Output: ["npmjs", "package/request"]; 

Input: "http://www.github.com/tensorflow/tensorflow"
Output: ["github", "tensorflow/tensorflow_something"]. 
*/
// Parses a URL and returns its root domain and subdirectories
pub fn get_data(url: &str) -> Vec<String> {
    let regular_expression = Regex::new(r"^https?://(www\.)?(npmjs|github)\.com/(.+)").unwrap();
    if is_valid(url, &regular_expression) {
        // Returns all groups the regex captured in the URL
        let matches = regular_expression.captures(url).unwrap();
        // Root Domain (Group 2) (github or npmjs)
        let domain = &matches[2];
        // Subdirectories (Group 3) (subfolders/packages)
        let subdirs = &matches[3];
        return vec![domain.to_string(), subdirs.to_string()]
    } else {
        eprintln!("Invalid URL: {}", url);
        std::process::exit(1);
    }
}
