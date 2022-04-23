use std::env;

use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    println!("start");

    let bot = Bot::new(env::var("spy-token").unwrap()).auto_send();

    teloxide::repl(bot, |message| async move {
        println!("message received");
        message.answer("hello").await?;
        respond(())
    })
        .await;
}