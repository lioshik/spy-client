use teloxide::prelude::*;
use teloxide::types;
use std::env;
use std::fs::File;
use std::io::{Cursor, Write};
use teloxide::types::InputFile;

pub struct TelegramClient {
    bot: Bot,
    chat_id: i64
}

impl TelegramClient {
    pub async fn from_env(message: String) -> TelegramClient {
        let bot = Bot::new(env::var("spy-token").unwrap());
        let chat_id = env::var("spy-user-id").unwrap().parse::<i64>().unwrap();
        if message != "" {
            bot.send_message(chat_id, message).send().await;
        }
        TelegramClient {
            bot,
            chat_id
        }
    }

    pub async fn send_text(&self, text: String) {
        self.bot.send_message(self.chat_id, text).send().await;
    }

    pub async fn send_image(&self, vec: Vec<u8>) {
        self.bot.send_photo(self.chat_id, InputFile::memory(vec)).send().await;
    }

    pub async fn send_image_withcaption(&self, vec: Vec<u8>, caption: String) {
        self.bot.send_photo(self.chat_id, InputFile::memory(vec)).caption(caption).send().await;
    }
}
