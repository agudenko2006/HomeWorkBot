use std::path::{Path, PathBuf};

use clap::Parser;
use color_eyre::eyre::Result;
use homeworkbot::{parse_homework, Assignment};
use teloxide::{prelude::*, utils::command::BotCommands};

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
    #[command(description = "show the version and help")]
    Start,
    #[command(description = "display this text")]
    Help,
    #[command(description = "show all the homework that's not past the due date")]
    All,
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

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;

    Ok(())
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Start => {
            bot.send_message(msg.chat.id, "Hello world. I am v0.0.0")
                .await?;
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::All => {
            let homework = parse_homework(Path::new("demo"));
            for assignment in homework {
                bot.send_message(msg.chat.id, form_message(&assignment))
                    .await?;
            }
        }
        Command::Today => {
            bot.send_message(msg.chat.id, "Not implemented yet").await?;
        }
        Command::Tomorrow => {
            bot.send_message(msg.chat.id, "Not implemented yet").await?;
        }
    };

    Ok(())
}

fn form_message(assignment: &Assignment) -> String {
    if assignment.tasks.len() == 1 {
        format!("{}: {}", assignment.subject, assignment.tasks[0].body)
    } else {
        let tasks: String = assignment
            .tasks
            .iter()
            .map(|task| format!("+ {}\n", task.body))
            .collect();

        format!("{}:\n{}", assignment.subject, tasks)
    }
}
