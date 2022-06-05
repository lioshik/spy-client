use teloxide::prelude::*;
use teloxide::types;
use std::env;
use std::fs::File;
use std::io::{Cursor, Write};
use teloxide::types::InputFile;

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

    pub async fn send_text(&self, text: String) {
        self.bot.send_message(self.chat_id, text).await;
    }

    pub async fn send_image(&self, vec: Vec<u8>) {
        self.bot.send_photo(self.chat_id, InputFile::memory(vec)).await;
    }

    pub async fn send_image_withcaption(&self, vec: Vec<u8>, caption: String) {
        self.bot.send_photo(self.chat_id, InputFile::memory(vec)).caption(caption).await;
    }
}
