use crate::SqlitePool;
use crate::input;
use crate::models::user::User;

pub async fn prompt_add_user(pool: &SqlitePool) -> anyhow::Result<User> {
    let first_name = input::name_input("First")?;
    let last_name = input::name_input("Last")?;
    let user = User::create_user(pool, &first_name, &last_name).await?;

    Ok(user)
}
