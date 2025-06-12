// INFO: Handles base input types tu build specific prompts

use capitalize::Capitalize;
use inquire::{Confirm, Select, Text};

pub fn text_input(prompt: &str) -> anyhow::Result<String> {
    Ok(Text::new(prompt)
        .with_help_message("Enter text and press ENTER")
        .prompt()?)
}

pub fn select(prompt: &str, options: Vec<&str>) -> anyhow::Result<String> {
    Ok(Select::new(prompt, options).prompt()?.to_string())
}

pub fn confirm(prompt: &str) -> anyhow::Result<bool> {
    Ok(Confirm::new(prompt).with_default(true).prompt()?)
}

pub fn name_input(which: &str) -> anyhow::Result<String> {
    Ok(Text::new(&format!("{} name", which))
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
