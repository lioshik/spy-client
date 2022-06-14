use std::cmp::min;
use teloxide::prelude::*;
use teloxide::types;
use std::env;
use std::fs::File;
use std::io::{Cursor, Write};
use teloxide::types::InputFile;
use winapi::um::winsock2::send;

pub struct TelegramClient {
    pub bot: AutoSend<Bot>,
    chat_id: i64
}

impl TelegramClient {
    pub async fn from_env(message: String) -> TelegramClient {
        let bot = Bot::new(env::var("spy-token").unwrap()).auto_send();
        let chat_id = env::var("spy-user-id").unwrap().parse::<i64>().unwrap();
        if message != "" {
            bot.send_message(chat_id, message).await;
        }
        TelegramClient {
            bot,
            chat_id
        }
    }

    pub async fn send_text(&self, text: &String) {
        const MESSAGE_MAX_SIZE: usize  = 3500;
        let text_vec = text.chars().collect::<Vec<_>>();
        let message_count = (text_vec.len() + MESSAGE_MAX_SIZE - 1) / MESSAGE_MAX_SIZE;
        for i in 0..message_count {
            if (message_count == 1) {
                self.bot.send_message(self.chat_id,
                                      &text.to_string()).await;
            } else {
                let part_of_text = format!("[message is too long, show part {} of {}]\n\n{}", i + 1, message_count,
                                           text_vec[i * MESSAGE_MAX_SIZE..min(text_vec.len(), (i + 1) * MESSAGE_MAX_SIZE)].iter().cloned().collect::<String>());
                self.bot.send_message(self.chat_id, part_of_text).await;
            }
        }
    }

    pub async fn send_image(&self, vec: Vec<u8>) {
        self.bot.send_photo(self.chat_id, InputFile::memory(vec)).await;
    }

    pub async fn send_image_withcaption(&self, vec: Vec<u8>, caption: String) {
        self.bot.send_photo(self.chat_id, InputFile::memory(vec)).caption(caption).await;
    }
}
