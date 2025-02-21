# README.md

# Pomodoro Timer

A simple Pomodoro timer application built in Rust. This project helps you manage your time effectively by using the Pomodoro technique, which involves working in focused intervals followed by short breaks.

## Project Structure

- `src/main.rs`: Entry point of the application that initializes the timer and starts the main loop.
- `src/timer.rs`: Contains the `Timer` struct with methods to control the timer functionality.
- `src/config.rs`: Holds configuration settings such as work duration and break duration.
- `src/utils.rs`: Utility functions for time formatting and other helper functions.
- `Cargo.toml`: Configuration file for Cargo, specifying project metadata and dependencies.
- `Cargo.lock`: Automatically generated file that locks the versions of dependencies.

## Setup Instructions

1. Ensure you have Rust and Cargo installed on your machine. You can install them from [rustup.rs](https://rustup.rs/).
2. Clone the repository:
   ```
   git clone <repository-url>
   ```
3. Navigate to the project directory:
   ```
   cd pomodoro-timer
   ```
4. Build the project:
   ```
   cargo build
   ```
5. Run the application:
   ```
   cargo run
   ```

## Usage

Once the application is running, you can start the timer, pause it, or reset it according to your needs. Adjust the configuration settings to customize your work and break durations.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.