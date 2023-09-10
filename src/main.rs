use std::path::Path;

use color_eyre::eyre::Result;
use homeworkbot::parse_homework;
use teloxide::{prelude::*, utils::command::BotCommands};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text")]
    Help,
    #[command(description = "show homework from today")]
    Today,
    #[command(description = "show homework for tomorrow")]
    Tomorrow,
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    log::info!("Starting HomeWorkBot...");
    color_eyre::install()?;

    dbg!(parse_homework(Path::new("demo")));

    return Ok(());

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;

    Ok(())
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Today => bot.send_message(msg.chat.id, "Not implemented yet").await?,
        Command::Tomorrow => bot.send_message(msg.chat.id, "Not implemented yet").await?,
    };

    Ok(())
}
