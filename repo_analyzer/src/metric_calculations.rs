fn normalize(value: f32, range: f32) -> f32 {
    value / range
}

pub fn get_ramp_up_time(codebase_length: &str) -> f32 {
    let codebase_length = match codebase_length.parse::<f32>() {
        Ok(n) => n,
        Err(_) => 0.0
    };
    normalize(codebase_length, 100000.0)
}

pub fn get_correctness(opened_issues: &str) -> f32 {
    let opened_issues = match opened_issues.parse::<f32>() {
        Ok(n) => n,
        Err(_) => 0.0
    };
    normalize(opened_issues, 2000.0)
}

pub fn get_bus_factor(number_of_forks: &str) -> f32 {
    let number_of_forks = match number_of_forks.parse::<f32>() {
        Ok(n) => n,
        Err(_) => 0.0
    };
    normalize(number_of_forks, 1000.0)
}

pub fn get_license(license: &str) -> f32 {
    0.0
}

pub fn get_responsive_maintainer() -> f32 {
    -1.0
}

pub fn get_overall(metrics: &[f32]) -> f32 {
    let sum: f32 = metrics.iter().sum();
    sum / metrics.len() as f32
}
