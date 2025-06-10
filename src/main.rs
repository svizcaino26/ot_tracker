// use anyhow::Context;
use capitalize::Capitalize;
use inquire::{Confirm, Select, Text, error::InquireError};
use sqlx::FromRow;
use sqlx::sqlite::SqlitePool;
use std::io::{self, Read};
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

mod handlers;
mod models;

use handlers::user_handler::*;

#[derive(Debug, FromRow)]
struct Overtime {
    ot_id: Option<i64>,
    user_id: Option<i64>,
    start_time: Option<OffsetDateTime>,
    end_time: Option<OffsetDateTime>,
    description: Option<String>,
}

impl Overtime {
    async fn start_tracking(
        pool: &SqlitePool,
        first_name: &str,
        last_name: &str,
        description: &str,
    ) -> anyhow::Result<Overtime> {
        let user_id = User::get_user(pool, first_name, last_name).await?.user_id;

        let now = get_current_time()?;

        let ot = sqlx::query_as!(
            Overtime,
            r#"
        INSERT INTO overtime (user_id, start_time, description)
        VALUES(?1, ?2, ?3)
        RETURNING ot_id, user_id, start_time as "start_time: OffsetDateTime",
        end_time as "end_time: OffsetDateTime", description
        "#,
            user_id,
            now,
            description
        )
        .fetch_one(pool)
        .await?;

        Ok(ot)
    }

    async fn end_tracking(&mut self, pool: &SqlitePool) -> anyhow::Result<()> {
        let now = get_current_time()?;

        sqlx::query!(
            r#"
        UPDATE overtime
        SET end_time = ?1
        WHERE ot_id = ?2
        "#,
            now,
            self.ot_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

fn get_current_time() -> anyhow::Result<String> {
    let now = OffsetDateTime::now_local()
        .map_err(|e| anyhow::anyhow!("Failed to get local time: {}", e))?
        .format(&Rfc3339)
        .map_err(|e| anyhow::anyhow!("Failed to format time: {}", e))?;
    Ok(now)
}

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
                                }
                                Err(e) => {
                                    println!("{e}");
                                    let _ = io::stdin().read(&mut [0u8]).unwrap();
                                    continue;
                                }
                            }
                        } else {
                            panic!("Error getting last name from input")
                        }
                    } else {
                        panic!("Error getting first name from input")
                    }

                    let _ = io::stdin().read(&mut [0u8]).unwrap();
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
                    let _ = io::stdin().read(&mut [0u8]).unwrap();
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
                    let _ = io::stdin().read(&mut [0u8]).unwrap();
                }
                "check overtime" => println!("{choice} selected"),
                "list users" => {
                    let users = User::list_users(&pool).await?;
                    users
                        .iter()
                        .for_each(|user| println!("{} {}", user.first_name, user.last_name));
                    let _ = io::stdin().read(&mut [0u8]).unwrap();
                }
                "quit" => break,
                _ => (),
            },
            Err(_) => println!("There was an error, please try again"),
        }
    }
    Ok(())
}
