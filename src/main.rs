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
            MenuOption::GetOvertime => {
                match overtime_prompts::prompt_get_overtime(&pool).await {
                    Ok(ot) => {
                        let total_ot = utils::format_duration(ot);
                        println!("Total accummulated overtime: {}", total_ot);
                    }
                    Err(e) => eprintln!("Error: {}", e),
                }
                utils::pause();
            }
            MenuOption::Quit => break,
        }
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
