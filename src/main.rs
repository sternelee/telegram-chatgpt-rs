use teloxide::prelude::*;
use chatgpt::prelude::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    // 创建机器人
    let bot = Bot::from_env();

    // 连接 chatgpt
    let token: String = std::env::var("SESSION_TOKEN").unwrap();
    let mut client = ChatGPT::new(token)?;
    client.refresh_token().await?;

    let mut conversation = client.new_conversation();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        let text = msg.text();
        let response: String = conversation.send_message(&client, text).await?;
        bot.send_message(msg.chat.id, response).await?;
        Ok(())
    })
    .await;
}
