use std::error::Error;
use teloxide::Bot;
use teloxide::prelude2::{AutoSend, Message, Requester};
use teloxide::utils::command::BotCommand;
use crate::TelegramClient;

pub struct TelegramFileClient {
    pub client: TelegramClient
}

impl TelegramFileClient {
    pub async fn from_env(message: String) -> TelegramFileClient {
        let client = TelegramClient::from_env(message).await;
        TelegramFileClient {
            client
        }
    }

    pub async fn start(self) {
        teloxide::repls2::commands_repl(self.client.bot, answer, Command::ty()).await;
    }
}

#[derive(BotCommand, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "show directory")]
    ShowDir,
    #[command(description = "send file")]
    SendFile
}

async fn answer(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::ShowDir => {
            bot.send_message(message.chat.id, "show dir comand").await?;
        }
        Command::SendFile => {
            bot.send_message(message.chat.id, "send file comand").await?;
        }
    };

    Ok(())
}