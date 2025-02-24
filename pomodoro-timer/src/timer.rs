// src/timer.rs

use std::{thread, time::Duration};
use std::io::{self, Write};

pub struct Timer;

impl Timer {
    pub fn new() -> Self {
        Timer
    }

    pub fn start(&mut self, minutes: u64) {
        let total_seconds = minutes * 60;
        
        for seconds_left in (0..=total_seconds).rev() {
            let minutes_left = seconds_left / 60;
            let seconds = seconds_left % 60;
            
            // Clear the current line and show countdown
            print!("\r⏳ {:02}:{:02} remaining", minutes_left, seconds);
            io::stdout().flush().unwrap();
            
            thread::sleep(Duration::from_secs(1));
        }
        println!(); // New line after countdown finishes
    }

    pub fn wait(&self) {
        // No longer needed as start() handles the waiting
    }
}