mod timer;
mod config;

use timer::Timer;
use config::Config;

fn main() {
    let mut timer = Timer::new();
    let config = Config::load();
    let mut session_count = 1;

    loop {
        println!("\n🍅 Session {} 🍅", session_count);
        println!("------------------");
        
        // Work session
        println!("📚 Starting work session...");
        timer.start(config.work_duration);
        println!("✨ Work session completed!");
        
        // Break session
        println!("\n☕ Time for a break!");
        timer.start(config.break_duration);
        println!("🔄 Break completed! Get ready for the next session.");
        
        session_count += 1;
    }
}