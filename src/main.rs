use capitalize::Capitalize;
use inquire::{Confirm, Select, Text, error::InquireError};
use sqlx::FromRow;
use sqlx::sqlite::SqlitePool;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

mod handlers;
mod models;
mod prompts;
mod ui;
mod utils;

use handlers::overtime_handler::*;
use handlers::user_handler::*;
use prompts::overtime_prompts;
use prompts::user_prompts;
use ui::input;
use ui::menu::MenuOption;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let pool = SqlitePool::connect("data.db").await?;

    loop {
        clearscreen::clear().expect("failed to clear screeen");
        match MenuOption::prompt()? {
            MenuOption::AddUser => {
                match user_prompts::prompt_add_user(&pool).await {
                    Ok(_user) => println!("User added successfully"),
                    Err(e) => eprintln!("Error: {}", e),
                }
                utils::pause();
            }

            MenuOption::RemoveUser => {
                match user_prompts::prompt_remove_user(&pool).await {
                    Ok(()) => println!("User removed successfully"),
                    Err(e) => eprintln!("Error: {}", e),
                }
                utils::pause();
            }
            MenuOption::ListUsers => {
                User::display_user_list(&pool).await?;
                utils::pause();
            }
            MenuOption::AddOvertime => {
                match overtime_prompts::prompt_add_overtime(&pool).await {
                    Ok(()) => println!("Overtime successfully recorded"),
                    Err(e) => eprintln!("Error: {}", e),
                }
                utils::pause();
            }
            MenuOption::GetOvertime => println!("get overtime selected"),
            MenuOption::Quit => break,
        }

        // let ans: Result<&str, InquireError> =
        //     Select::new("What do you want to do?", options).prompt();
        // match ans {
        //     Ok(choice) => match choice {
        //         "get total overtime" => {
        //             let total_overtime = Overtime::calculate_total_overtime(&pool).await?;
        //             let total_overtime = utils::format_duration(total_overtime);
        //             println!("Total accmumulated overtime: {total_overtime}");
        //             utils::pause();
        //         }
        //         "get overtime by user" => {
        //             let users: Vec<String> = User::list_users(&pool)
        //                 .await?
        //                 .iter()
        //                 .map(|user| format!("{} {}", user.first_name, user.last_name))
        //                 .collect();
        //
        //             let user = Select::new("Select user", users).prompt();
        //
        //             if let Ok(user_split) = &user {
        //                 let mut user_split = user_split.split_whitespace();
        //                 let total_overtime = Overtime::calculate_overtime_by_user(
        //                     &pool,
        //                     user_split.next().unwrap(),
        //                     user_split.next().unwrap(),
        //                 )
        //                 .await?;
        //                 let total_overtime = utils::format_duration(total_overtime);
        //                 println!(
        //                     "Total accmumulated overtime for {}: {}",
        //                     user.unwrap(),
        //                     total_overtime
        //                 );
        //             }
        //
        //             utils::pause();
        //         }
        //     },
        //     Err(_) => println!("There was an error, please try again"),
        // }
    }
    Ok(())
}

fn date_select_menu() {
    use inquire::DateSelect;

    let date = DateSelect::new("When do you want to travel?")
        .with_help_message("Possible flights will be displayed according to the selected date")
        .prompt();

    match date {
        Ok(date) => println!("{}", date),
        Err(_) => println!("There was an error in the system."),
    }
}
