pub fn get_ramp_up_time(url: &str) -> f32 {
    let docsLength = getDocsLength(url);
    let codebaseLength = getCodebaseLength(url);
    docsLength as f32 / codebaseLength  as f32
}

pub fn get_correctness(url: &str) -> f32 {
    let closedBugs = getClosedBugs(url);
    let openedIssues = getOpenedIssues(url);
    closedBugs as f32 / openedIssues  as f32
}

pub fn get_responsive_maintainer(url: &str) -> f32 {
    0.0
}

pub fn get_bus_factor(url: &str) -> f32 {
    0.0
}

pub fn get_license(url: &str) -> f32 {
    isCompatible(url)
}

pub fn get_overall(metrics: &[f32]) -> f32 {
    let sum: f32 = metrics.iter().sum();
    sum / metrics.len() as f32
}



// HELPER FUNCTIONS

// This should take in the url and return the number of lines in the module's documentation.
// If no documentation is present it should return 0.
pub fn getDocsLength(url: &str) -> f32 {
    1000.0 // placeholder value
}

// This should take in the url and return the number of lines in the module's codebase.
pub fn getCodebaseLength(url: &str) -> f32 {
    20000.0 // placeholder value
}

// This should take in the url and return the number of closed bugs.
pub fn getClosedBugs(url: &str) -> f32 {
    100.0 // placeholder value
}

// This should take in the url and return the number of opened issues.
pub fn getOpenedIssues(url: &str) -> f32 {
    120.0 // placeholder value
}

// Returns 1 if the module is compatible with ACME's license, 0 otherwise.
fn isCompatible(url: &str) -> f32 {
    0.0
}