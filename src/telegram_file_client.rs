use std::borrow::Borrow;
use std::cmp::min;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use teloxide::Bot;
use teloxide::prelude2::{AutoSend, Message, Requester};
use teloxide::types::InputFile;
use teloxide::utils::command::BotCommand;
use crate::TelegramClient;

pub struct TelegramFileClient {
    pub client: TelegramClient,
}

impl TelegramFileClient {
    pub async fn from_env(message: String) -> TelegramFileClient {
        let client = TelegramClient::from_env(message).await;
        client.bot.set_my_commands([
            teloxide::types::BotCommand { command: "showdir".to_string(), description: "/showdir <path to directory>".to_string() },
            teloxide::types::BotCommand { command: "sendfile".to_string(), description: "/sendfile <path to file>".to_string() },
        ].into_iter()).await;
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
    SendFile,
}

async fn answer(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::ShowDir => {
            let message_text = message.text().unwrap();
            if message_text.len() <= 9 {
                bot.send_message(message.chat.id, "Error: empty path").await?;
            } else {
                let path = &message_text[9..];
                bot.send_message(message.chat.id, format!("Scanning \"{}\".....", path)).await?;
                if Path::new(path).exists() {
                    if (Path::new(path).is_dir()) {
                        let result = fs::read_dir(path);
                        if (result.is_ok()) {
                            let paths = result.unwrap();
                            let mut text_files = "".to_string();
                            let mut text_dirs = "".to_string();
                            for path in paths {
                                if (path.is_err()) {} else {
                                    if (Path::new(path.as_ref().unwrap().path().to_str().unwrap()).is_dir()) {
                                        text_dirs.push_str(&*format!("📂{}",
                                                                     path.as_ref().unwrap().path().to_str().unwrap())
                                        );
                                        text_dirs.push_str("\n\n");
                                    } else {
                                        let metadata = fs::metadata(Path::new(path.as_ref().unwrap().path().to_str().unwrap()));
                                        let m_bytes: String;
                                        if metadata.is_ok() {
                                            let bytes = metadata.unwrap().len();
                                            m_bytes = format!("[{:.2} MBytes]", (bytes as f64) / 1024.0f64 / 1024.0f64);
                                        } else {
                                            m_bytes = "[size unknown]".to_string();
                                        }
                                        text_files.push_str(&*format!("📄{}  {}",
                                                                      path.as_ref().unwrap().path().to_str().unwrap(),
                                                                      m_bytes)
                                        );
                                        text_files.push_str("\n\n");
                                    }
                                }
                            }
                            let mut text = "".to_string();
                            text.push_str(&*text_dirs);
                            text.push_str(&*text_files);
                            if text.len() == 0 {
                                bot.send_message(message.chat.id, "Error: empty directory").await;
                            } else {
                                const MESSAGE_MAX_SIZE: usize = 3500;
                                let text_vec = text.chars().collect::<Vec<_>>();
                                let message_count = (text_vec.len() + MESSAGE_MAX_SIZE - 1) / MESSAGE_MAX_SIZE;
                                for i in 0..message_count {
                                    if (message_count == 1) {
                                        bot.send_message(message.chat.id,
                                                         &text.to_string()).await;
                                    } else {
                                        let part_of_text = format!("[message is too long, show part {} of {}]\n\n{}", i + 1, message_count,
                                                                   text_vec[i * MESSAGE_MAX_SIZE..min(text_vec.len(), (i + 1) * MESSAGE_MAX_SIZE)].iter().cloned().collect::<String>());
                                        bot.send_message(message.chat.id, part_of_text).await;
                                    }
                                }
                            }
                        } else {
                            bot.send_message(message.chat.id, "Error: unable to read directory").await?;
                        }
                    } else {
                        bot.send_message(message.chat.id, "Error: path leads to file, not direcotory").await?;
                    }
                } else {
                    bot.send_message(message.chat.id, "Error: path does not exists").await?;
                }
            }
        }
        Command::SendFile => {
            let message_text = message.text().unwrap();
            if message_text.len() <= 10 {
                bot.send_message(message.chat.id, "Error: empty path").await?;
            } else {
                let path = &message_text[10..];
                bot.send_message(message.chat.id, format!("Sending \"{}\".....", path)).await?;
                if Path::new(path).exists() {
                    if Path::new(path).is_file() {
                        let result = bot.send_document(message.chat.id, InputFile::file(
                            PathBuf::from(path))).await;
                        if result.is_err() {
                            bot.send_message(message.chat.id, format!("Error: {}", result.err().unwrap().to_string())).await?;
                        }
                    } else {
                        bot.send_message(message.chat.id, "Error: path leads to directory, not file").await?;
                    }
                } else {
                    bot.send_message(message.chat.id, "Error: path does not exists").await?;
                }
            }
        }
    };

    Ok(())
}