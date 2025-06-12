use inquire::{InquireError, Select};

pub enum MenuOption {
    AddUser,
    RemoveUser,
    ListUsers,
    AddOvertime,
    GetOvertime,
    Quit,
}

impl MenuOption {
    pub fn prompt() -> Result<MenuOption, InquireError> {
        let options = vec![
            "add user",
            "remove user",
            "list users",
            "add overtime",
            "get overtime",
            "quit",
        ];

        let choice = Select::new("Select an option from below", options).prompt()?;

        match choice {
            "add user" => Ok(MenuOption::AddUser),
            "remove user" => Ok(MenuOption::RemoveUser),
            "list users" => Ok(MenuOption::ListUsers),
            "add overtime" => Ok(MenuOption::AddOvertime),
            "get overtime" => Ok(MenuOption::GetOvertime),
            _ => Ok(MenuOption::Quit),
        }
    }
}
