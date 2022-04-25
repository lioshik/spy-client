use teloxide::prelude::*;
use teloxide::types;
use std::env;

pub struct TelegramClient {
    sender: AutoSend<Bot>,
    chat_id: i64
}

impl TelegramClient {
    pub async fn from_env() -> TelegramClient {
        let bot = Bot::new(env::var("spy-token").unwrap());
        let sender = bot.auto_send();
        let chat_id = env::var("spy-user-id").unwrap().parse::<i64>().unwrap();
        sender.send_message(chat_id, "session started").await;
        TelegramClient {
            sender,
            chat_id
        }
    }

    pub async fn send_text(&self, text: String) {
        self.sender.send_message(self.chat_id, text).await;
    }
}
