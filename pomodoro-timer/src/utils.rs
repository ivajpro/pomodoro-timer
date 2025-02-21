// This file contains utility functions that assist with time formatting and other helper functions used throughout the application.

pub fn format_time(seconds: u64) -> String {
    let minutes = seconds / 60;
    let seconds = seconds % 60;
    format!("{:02}:{:02}", minutes, seconds)
}

pub fn parse_time(input: &str) -> Result<u64, String> {
    let parts: Vec<&str> = input.split(':').collect();
    if parts.len() != 2 {
        return Err("Invalid time format. Use MM:SS".to_string());
    }
    
    let minutes: u64 = parts[0].parse().map_err(|_| "Invalid minutes".to_string())?;
    let seconds: u64 = parts[1].parse().map_err(|_| "Invalid seconds".to_string())?;
    
    Ok(minutes * 60 + seconds)
}