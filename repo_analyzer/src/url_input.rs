pub fn run(url: &str) -> Box<String> {
    println!("Hello from url_input.rs in the Rust library!");
    return Box::new(String::from("from url_input.rs"));
}

use regex::Regex;

/// Returns true if there was a match to the regex
pub fn is_valid(url: &str, regex: &Regex) -> bool {
    regex.is_match(url)
}

/// Parses a URL and returns its root domain and subdirectories
pub fn get_data(url: &str) -> Vec<String> {
    let regular_expression = Regex::new(r"^https?://(www.)?(npmjs|github).com/(.+)").unwrap();
    if is_valid(url, &regular_expression) {
        /// Returns all groups the regex captured in the URL
        let matches = regular_expression.captures(url).unwrap();
        /// Root Domain (Group 2)
        let domain = &matches[2];
        /// Subdirectories (Group 3)
        let subdirs = &matches[3];
        return vec![domain.to_string(), subdirs.to_string()]
    } else {
        eprintln!("Invalid URL: {}", url);
        std::process::exit(1);
    }
}
