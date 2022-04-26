use teloxide::prelude::*;
use teloxide::types;
use std::env;
use teloxide::types::InputFile;

pub struct TelegramClient {
    bot: Bot,
    chat_id: i64
}

impl TelegramClient {
    pub async fn from_env() -> TelegramClient {
        let bot = Bot::new(env::var("spy-token").unwrap());
        let chat_id = env::var("spy-user-id").unwrap().parse::<i64>().unwrap();
        bot.send_message(chat_id, "session started").send().await;
        TelegramClient {
            bot,
            chat_id
        }
    }

    pub async fn send_text(&self, text: String) {
        self.bot.send_message(self.chat_id, text).send().await;
    }

    pub async fn send_image(&self, filename: &String) {
        self.bot.send_photo(self.chat_id, InputFile::file(filename)).send().await;
    }

    pub async fn send_image_withcaption(&self, filename: &String, caption: String) {
        self.bot.send_photo(self.chat_id, InputFile::file(filename)).caption(caption).send().await;
    }
}
