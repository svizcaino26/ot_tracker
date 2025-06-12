// INFO: Handles base input types tu build specific prompts

use crate::SqlitePool;
use crate::User;
use capitalize::Capitalize;
use inquire::{Confirm, Select, Text};

pub fn description_input(prompt: &str) -> anyhow::Result<String> {
    Ok(Text::new(prompt)
        .with_help_message("Enter a description to start recording")
        .prompt()?)
}

pub fn select(prompt: &str, options: Vec<&str>) -> anyhow::Result<String> {
    Ok(Select::new(prompt, options).prompt()?.to_string())
}

pub fn confirm(prompt: &str) -> anyhow::Result<bool> {
    Ok(Confirm::new(prompt).with_default(true).prompt()?)
}

pub fn name_input(which: &str) -> anyhow::Result<String> {
    Ok(Text::new(&format!("{} name:", which))
        // TODO: add error handling for empty strings here
        // .with_validator(|input: &str| {
        //     if input.trim().is_empty() {
        //         // Err(&format!("{} name cannot be empty", which).into())
        //         Err("name cannot be empty".into())
        //     } else {
        //         Ok(())
        //     }
        // })
        .prompt()?
        .trim()
        .to_string()
        .capitalize())
}

pub async fn user_select(pool: &SqlitePool) -> anyhow::Result<Option<String>> {
    let users: Vec<String> = User::list_users(pool)
        .await?
        .iter()
        .map(|user| format!("{} {}", user.first_name, user.last_name))
        .collect();

    Ok(Select::new("Select user", users)
        .with_help_message("Press ESC to skip")
        .prompt_skippable()?)
}
