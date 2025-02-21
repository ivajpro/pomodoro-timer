// src/config.rs

use std::fs::File;
use std::io::{self, Read, Write};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub work_duration: u64, // in seconds
    pub break_duration: u64, // in seconds
}

impl Config {
    pub fn new(work_duration: u64, break_duration: u64) -> Self {
        Config {
            work_duration,
            break_duration,
        }
    }

    pub fn load() -> Self {
        Config {
            work_duration: 25,  // 25 minutes for work
            break_duration: 5,  // 5 minutes for break
        }
    }

    pub fn save(&self, file_path: &str) -> io::Result<()> {
        let mut file = File::create(file_path)?;
        let contents = serde_json::to_string(self)?;
        file.write_all(contents.as_bytes())?;
        Ok(())
    }
}