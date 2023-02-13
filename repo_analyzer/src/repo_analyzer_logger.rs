use std::fs::File;
use std::env;
use std::io::Write;

pub struct Logger {
    log_file: File,

}
impl Logger {

    pub fn fromEnvVar(var: &str) -> Result<Self, String>{
        let res = env::var(var);
        if res.is_err() {
            // println!("${} is not set in Enviromental Variables", var);
            return Err(res.err().unwrap().to_string())
        }

        Logger::new(&res.unwrap())
    }

    pub fn new(log_file: &str) -> Result<Self, String> {
        let mut file_res = File::open(log_file);
        if file_res.is_err() {
            return Err(file_res.unwrap_err().to_string());
        }

        Ok(Logger { log_file: file_res.unwrap() })
    }

    pub fn log_info(self: &mut Logger, msg: &str) {
        self.log_file.write(format!("[INFO]: {}", msg).as_ref());
    }
    pub fn log_warning(self: &mut Logger, msg: &str) {
        self.log_file.write(format!("[WARNING]: {}", msg).as_ref());
    }
    pub fn log_error(self: &mut Logger, msg: &str) {
        self.log_file.write(format!("[ERROR]: {}",msg).as_ref());
    }
}