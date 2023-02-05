


pub fn run(url: &str) -> Box<String> {
    println!("Hello from url_input.rs in the Rust library!");
    return Box::new(String::from("from url_input.rs"));
}

pub fn is_valid(url: &str) -> bool {
    return true;
}

pub fn get_data(url: &str) {

}