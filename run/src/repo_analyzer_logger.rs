use std::fs::{File, OpenOptions};
use std::process::Command;
use std::{env, fs};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use log::logger;

pub struct Logger {
    file_path: String,
    file: File
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
        // check if file path exists
        // find a better way to do this
        // let p = Path::new(log_file);
        // if !p.exists() {
        //     File::create(log_file);
        // }
        // let file_res = if !p.exists() { File::create(log_file) } else { File::open(log_file) };
        // if file_res.is_err() {
        //     let word = if !p.exists() { "create" } else { "open" };
        //     return Err(format!("Failed to {} file at {}", word, log_file));
        // }
        // let file = file_res.unwrap();

        let mut file_res = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file);
        
        if file_res.is_err(){
            return Err(format!("Unable to open file: {}", log_file));
        }

        let file = file_res.unwrap();

        Ok(Logger { file_path: log_file.to_owned(), file })
    }

    pub fn log_info(self: &mut Logger, msg: &str) {
        let full_msg = format!("[INFO]: {}\n", msg);
<<<<<<< HEAD
        //println!("{}",full_msg.to_owned());
        //fs::write(&self.file_path, full_msg);
        self.file.write_all(full_msg.as_bytes());
=======
        println!("{}",full_msg.to_owned());
        fs::write(&self.file_path, full_msg);

>>>>>>> 1d073151be5942aff9f5b8516ae46dac9752e980
        // self.log_file.write(full_msg.as_ref());
    }
    pub fn log_warning(self: &mut Logger, msg: &str) {
        let full_msg = format!("[WARNING]: {}\n", msg);
        //println!("{}",full_msg.to_owned());
        //fs::write(&self.file_path, full_msg);
        self.file.write_all(full_msg.as_bytes());
        // self.log_file.write(full_msg.as_ref());
    }
    pub fn log_error(self: &mut Logger, msg: &str) {
        let full_msg = format!("[ERROR]: {}\n",msg);
        //println!("{}",full_msg.to_owned());
        //fs::write(&self.file_path, full_msg);
        self.file.write_all(full_msg.as_bytes());
        // self.log_file.write(full_msg.as_ref());
    }
}