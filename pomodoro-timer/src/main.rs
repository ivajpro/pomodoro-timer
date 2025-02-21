mod timer;
mod config;

use timer::Timer;
use config::Config;

fn main() {
    // Initialize the Pomodoro timer
    let mut timer = Timer::new();

    // Load configuration settings
    let config = Config::load();

    // Start the main loop for the Pomodoro timer
    loop {
        // Start the timer with the configured work duration
        timer.start(config.work_duration);

        // Notify user that work session has started
        println!("Work session started for {} minutes.", config.work_duration);

        // Wait for the timer to finish
        timer.wait();

        // Notify user that work session has ended
        println!("Work session ended. Take a break!");

        // Start the break timer
        timer.start(config.break_duration);
        println!("Break session started for {} minutes.", config.break_duration);

        // Wait for the break timer to finish
        timer.wait();

        // Notify user that break session has ended
        println!("Break session ended. Get ready for the next work session!");
    }
}