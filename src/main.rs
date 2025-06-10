use capitalize::Capitalize;
use inquire::{Confirm, Select, Text, error::InquireError};
use sqlx::FromRow;
use sqlx::sqlite::SqlitePool;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

mod handlers;
mod models;
mod utils;

use handlers::overtime_handler::*;
use handlers::user_handler::*;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let pool = SqlitePool::connect("data.db").await?;
    let exists = User::exists(&pool, "Stephen", "Vizcaino");
    println!("{}", exists.await.unwrap());

    let exists = User::exists(&pool, "Mr", "X");
    println!("{}", exists.await.unwrap());

    // let mut users: Vec<User> = Vec::new();

    loop {
        clearscreen::clear().expect("failed to clear screeen");
        let options: Vec<&str> = vec![
            "add user",
            "list users",
            "remove user",
            "add overtime",
            "check overtime",
            "quit",
        ];

        let ans: Result<&str, InquireError> =
            Select::new("What do you want to do?", options).prompt();
        match ans {
            Ok(choice) => match choice {
                "add user" => {
                    let first_name = Text::new("First name:").prompt();

                    if let Ok(first_name) = first_name {
                        let last_name = Text::new("Last name:").prompt();
                        if let Ok(last_name) = last_name {
                            match User::create_user(
                                &pool,
                                &first_name.capitalize(),
                                &last_name.capitalize(),
                            )
                            .await
                            {
                                Ok(user) => {
                                    println!("User {} {} added", user.first_name, user.last_name);
                                    utils::pause();
                                }
                                Err(e) => {
                                    println!("{e}");
                                    utils::pause();
                                    continue;
                                }
                            }
                        } else {
                            panic!("Error getting last name from input")
                        }
                    } else {
                        panic!("Error getting first name from input")
                    }

                    utils::pause();
                }
                "remove user" => {
                    let users: Vec<String> = User::list_users(&pool)
                        .await?
                        .iter()
                        .map(|user| format!("{} {}", user.first_name, user.last_name))
                        .collect();

                    let user_to_be_removed =
                        Select::new("Select the user to be removed", users).prompt();

                    if let Ok(user) = user_to_be_removed {
                        let mut user = user.split_whitespace();
                        User::delete_user(&pool, user.next().unwrap(), user.next().unwrap())
                            .await?;
                        println!("User successfully removed")
                    }
                    utils::pause();
                }
                "add overtime" => {
                    let users: Vec<String> = User::list_users(&pool)
                        .await?
                        .iter()
                        .map(|user| format!("{} {}", user.first_name, user.last_name))
                        .collect();

                    let user_to_track =
                        Select::new("Select user to start tracking", users).prompt();

                    // TODO: Add the prompt for description
                    let mut ot: Option<Overtime> = None;
                    if let Ok(user) = user_to_track {
                        let mut user = user.split_whitespace();
                        ot = Some(
                            Overtime::start_tracking(
                                &pool,
                                user.next().unwrap(),
                                user.next().unwrap(),
                                "",
                            )
                            .await?,
                        );
                    }

                    let ans = Confirm::new("Overtime tracking started")
                        .with_default(true)
                        .with_help_message("Press ENTER to end tracking")
                        .prompt();

                    match ans {
                        Ok(true) => {
                            if let Some(mut ot) = ot {
                                ot.end_tracking(&pool).await?;
                                println!("Tracking stopped, overtime recorded successfully");
                            }
                        }
                        Err(_) => println!("Error with questionnaire, try again later"),
                        _ => (),
                    }
                    utils::pause();
                }
                "check overtime" => println!("{choice} selected"),
                "list users" => {
                    let users = User::list_users(&pool).await?;
                    users
                        .iter()
                        .for_each(|user| println!("{} {}", user.first_name, user.last_name));
                    // let _ = io::stdin().read(&mut [0u8]).unwrap();
                    utils::pause();
                }
                "quit" => break,
                _ => (),
            },
            Err(_) => println!("There was an error, please try again"),
        }
    }
    Ok(())
}
