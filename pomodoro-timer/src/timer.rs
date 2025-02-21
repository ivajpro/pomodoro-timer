// src/timer.rs

use std::{thread, time::Duration};

pub struct Timer;

impl Timer {
    pub fn new() -> Self {
        Timer
    }

    pub fn start(&mut self, minutes: u64) {
        // Convert minutes to seconds
        let duration = Duration::from_secs(minutes * 60);
        thread::sleep(duration);
    }

    pub fn wait(&self) {
        // This is currently a no-op as the timer is blocking in start()
    }
}