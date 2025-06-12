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
            MenuOption::RemoveUser => println!("remove user selected"),
            MenuOption::ListUsers => println!("list users selected"),
            MenuOption::AddOvertime => println!("add overtime selected"),
            MenuOption::GetOvertime => println!("get overtime selected"),
            MenuOption::Quit => break,
        }

        // let ans: Result<&str, InquireError> =
        //     Select::new("What do you want to do?", options).prompt();
        // match ans {
        //     Ok(choice) => match choice {
        //         "remove user" => {
        //             let users: Vec<String> = User::list_users(&pool)
        //                 .await?
        //                 .iter()
        //                 .map(|user| format!("{} {}", user.first_name, user.last_name))
        //                 .collect();
        //
        //             let user_to_be_removed =
        //                 Select::new("Select the user to be removed", users).prompt();
        //
        //             if let Ok(user) = user_to_be_removed {
        //                 let mut user = user.split_whitespace();
        //                 User::delete_user(&pool, user.next().unwrap(), user.next().unwrap())
        //                     .await?;
        //                 println!("User successfully removed")
        //             }
        //             utils::pause();
        //         }
        //         "add overtime" => {
        //             let users: Vec<String> = User::list_users(&pool)
        //                 .await?
        //                 .iter()
        //                 .map(|user| format!("{} {}", user.first_name, user.last_name))
        //                 .collect();
        //
        //             let user_to_track =
        //                 Select::new("Select user to start tracking", users).prompt();
        //
        //             // TODO: Add the prompt for description
        //             let mut ot: Option<Overtime> = None;
        //             if let Ok(user) = user_to_track {
        //                 let mut user = user.split_whitespace();
        //                 ot = Some(
        //                     Overtime::start_tracking(
        //                         &pool,
        //                         user.next().unwrap(),
        //                         user.next().unwrap(),
        //                         "",
        //                     )
        //                     .await?,
        //                 );
        //             }
        //
        //             let ans = Confirm::new("Overtime tracking started")
        //                 .with_default(true)
        //                 .with_help_message("Press ENTER to end tracking")
        //                 .prompt();
        //
        //             match ans {
        //                 Ok(true) => {
        //                     if let Some(mut ot) = ot {
        //                         ot.end_tracking(&pool).await?;
        //                         println!("Tracking stopped, overtime recorded successfully");
        //                     }
        //                 }
        //                 Err(_) => println!("Error with questionnaire, try again later"),
        //                 _ => (),
        //             }
        //             utils::pause();
        //         }
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
        //         "list users" => {
        //             let users = User::list_users(&pool).await?;
        //             users
        //                 .iter()
        //                 .for_each(|user| println!("{} {}", user.first_name, user.last_name));
        //             utils::pause();
        //         }
        //         "quit" => break,
        //         "date" => {
        //             date_select_menu();
        //             utils::pause();
        //         }
        //         _ => (),
        //     },
        //     Err(_) => println!("There was an error, please try again"),
        // }
    }
    Ok(())
}

fn date_select_menu() {
    // use chrono::{NaiveDate, Weekday};
    use inquire::DateSelect;

    let date = DateSelect::new("When do you want to travel?")
        // .with_starting_date(NaiveDate::from_ymd(2021, 8, 1))
        // .with_min_date(NaiveDate::from_ymd(2021, 8, 1))
        // .with_max_date(NaiveDate::from_ymd(2021, 12, 31))
        // .with_week_start(Weekday::Mon)
        .with_help_message("Possible flights will be displayed according to the selected date")
        .prompt();

    match date {
        Ok(date) => println!("{}", date),
        Err(_) => println!("There was an error in the system."),
    }
}
