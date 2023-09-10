use std::{collections::HashMap, path::PathBuf};

use clap::Parser;
use color_eyre::eyre::Result;
use homeworkbot::{parse_config, parse_homework, Assignment};
use teloxide::{prelude::*, utils::command::BotCommands};

/// A telegram bot that sends the homework
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the Obsidian vault that stores tasks.
    /// Homework tasks are named `mmdd-sbj-homework` where sbj is the subject (3 letters)
    vault: PathBuf,

    /// Path to the config.toml file (read the README)
    config: PathBuf,
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

    // todo!("USE THIS!!!")
    let _args = Cli::parse();

    let bot = Bot::from_env();

    // todo!("implement a proper dispatcher")
    Command::repl(bot, answer).await;

    Ok(())
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    // todo!("optimize")
    let args = Cli::parse();
    let homework = parse_homework(&args.vault);
    let config = parse_config(&args.config);
    let subjects = config.subjects;

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
            for assignment in homework {
                bot.send_message(msg.chat.id, form_message(&assignment, &subjects))
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

fn form_message(assignment: &Assignment, subject_names: &HashMap<String, String>) -> String {
    let subject = subject_names
        .get(&assignment.subject)
        .unwrap_or(&assignment.subject);

    if assignment.tasks.len() == 1 {
        format!("{}: {}", subject, assignment.tasks[0].body)
    } else {
        let tasks: String = assignment
            .tasks
            .iter()
            .map(|task| format!("+ {}\n", task.body))
            .collect();

        format!("{}:\n{}", subject, tasks)
    }
}
