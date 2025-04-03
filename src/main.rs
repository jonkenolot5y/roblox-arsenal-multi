use std::process::Command;
use std::thread;
use std::time::Duration;

mod features;
mod gui;

#[tokio::main]
async fn main() {
    gui::initialize_gui();
    features::load_features().await;
    loop {
        thread::sleep(Duration::from_millis(100));
    }
}