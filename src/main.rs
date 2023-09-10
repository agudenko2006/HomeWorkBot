use std::path::{Path, PathBuf};

use color_eyre::eyre::Result;
use homeworkbot::parse_homework;
use teloxide::{prelude::*, utils::command::BotCommands};
use clap::Parser;

/// A telegram bot that sends the homework
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the Obsidian vault that stores tasks.
    /// Homework tasks are named `mmdd-sbj-homework` where sbj is the subject (3 letters)
    vault: PathBuf,
}

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

    let args = Cli::parse();

    dbg!(parse_homework(&args.vault));

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
