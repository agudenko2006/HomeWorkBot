use std::{collections::HashMap, path::PathBuf};

use clap::Parser;
use color_eyre::eyre::Result;
use homeworkbot::{date::Date, parse_config, parse_homework, Assignment};
use teloxide::{prelude::*, types::ParseMode, utils::command::BotCommands};
#[macro_use]
extern crate log;

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
    #[command(description = "show this text")]
    Help,
    #[command(description = "show the changelog")]
    Changelog,
    #[command(description = "show all the homework that's not past the due date")]
    All,
    #[command(description = "show homework from today")]
    Today,
    #[command(description = "show homework for tomorrow")]
    Tomorrow,
    #[command(description = "show homework from the specified date (year-month-day or month-day)")]
    From(String), // todo!("use the Date type")
    #[command(description = "show homework due the specified date (year-month-day or month-day)")]
    To(String),
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    info!("Starting HomeWorkBot...");
    color_eyre::install()?;

    // todo!("USE THIS!!!")
    let _args = Cli::parse();

    let bot = Bot::from_env();

    // todo!("implement a proper dispatcher")
    Command::repl(bot, answer).await;

    Ok(())
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    info!("Received {}", msg.text().unwrap_or("an invalid message"));
    debug!("RECEIVED `{:?}`\n", msg);
    // todo!("optimize")
    let args = Cli::parse();
    let homework = parse_homework(&args.vault);
    let config = parse_config(&args.config);
    let subjects = config.subjects;

    // todo!("simplify this madness")
    match cmd {
        Command::Start => {
            bot.send_message(
                msg.chat.id,
                format!("{} V0.2.0 Online", config.name),
            )
            .await?;
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Changelog => {
            bot.send_message(
                msg.chat.id,
                format!("{} V0.2.0 Online", config.name),
            )
            .await?;
            bot.send_message(
                msg.chat.id,
                "Now I can use markdown, that means _formatted_ *text* and proper links\nFor example, [here's my source code](https://github.com/agudenko2006/homeworkbot)"
            ).parse_mode(ParseMode::MarkdownV2)
            .await?;
        }
        Command::All => {
            for assignment in homework
                .iter()
                .filter(|assignment| !assignment.to.had_passed())
            {
                let message = form_message(assignment, &subjects);
                debug!("SENDING `{}`", message);
                if (bot
                    .send_message(msg.chat.id, &message)
                    .parse_mode(ParseMode::MarkdownV2)
                    .await)
                    .is_err()
                {
                    bot.send_message(msg.chat.id, &message).await?;
                };
            }
        }
        Command::From(date) => {
            let date = match Date::from(&date) {
                Ok(date) => date,
                Err(_) => {
                    bot.send_message(
                        msg.chat.id,
                        "Please use the MM-DD format (e.g. 09-15 for Sep 15th)",
                    )
                    .await?;
                    return Ok(());
                }
            };

            for assignment in homework {
                if assignment.from != date {
                    continue;
                }

                let message = form_message(&assignment, &subjects);
                debug!("SENDING `{}`", message);
                if (bot
                    .send_message(msg.chat.id, &message)
                    .parse_mode(ParseMode::MarkdownV2)
                    .await)
                    .is_err()
                {
                    bot.send_message(msg.chat.id, &message).await?;
                };
            }
        }
        Command::To(date) => {
            let date = match Date::from(&date) {
                Ok(date) => date,
                Err(_) => {
                    bot.send_message(
                        msg.chat.id,
                        "Please use the MMDD format (e.g. 09-15 for Sep 15th)",
                    )
                    .await?;
                    return Ok(());
                }
            };

            for assignment in homework {
                if assignment.to != date {
                    continue;
                }

                let message = form_message(&assignment, &subjects);
                debug!("SENDING `{}`", message);
                if (bot
                    .send_message(msg.chat.id, &message)
                    .parse_mode(ParseMode::MarkdownV2)
                    .await)
                    .is_err()
                {
                    bot.send_message(msg.chat.id, &message).await?;
                };
            }
        }
        Command::Today => {
            let today = Date::now();

            for assignment in homework {
                if assignment.from != today {
                    continue;
                }

                let message = form_message(&assignment, &subjects);
                debug!("SENDING `{}`", message);
                if (bot
                    .send_message(msg.chat.id, &message)
                    .parse_mode(ParseMode::MarkdownV2)
                    .await)
                    .is_err()
                {
                    bot.send_message(msg.chat.id, &message).await?;
                };
            }
        }
        Command::Tomorrow => {
            let now = Date::now();
            let tomorrow = Date::new(now.0, now.1, now.2 + 1);

            for assignment in homework {
                if assignment.to != tomorrow {
                    continue;
                }

                let message = form_message(&assignment, &subjects);
                debug!("SENDING `{}`", message);
                if (bot
                    .send_message(msg.chat.id, &message)
                    .parse_mode(ParseMode::MarkdownV2)
                    .await)
                    .is_err()
                {
                    bot.send_message(msg.chat.id, &message).await?;
                };
            }
        }
    };

    info!("Responded");
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
            .map(|task| format!("\\+ {}\n", task.body))
            .collect();

        format!("{}:\n{}", subject, tasks)
    }
}
