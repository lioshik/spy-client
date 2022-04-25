#![windows_subsystem = "windows"]

mod telegram_client;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    let client = telegram_client::TelegramClient::from_env().await;
    client.send_text(String::from("some text")).await;
}