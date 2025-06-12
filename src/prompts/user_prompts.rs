use crate::SqlitePool;
use crate::input;
use crate::models::user::User;
use anyhow::bail;

pub async fn prompt_add_user(pool: &SqlitePool) -> anyhow::Result<User> {
    let first_name = input::name_input("First")?;
    let last_name = input::name_input("Last")?;
    let user = User::create_user(pool, &first_name, &last_name).await?;

    Ok(user)
}

pub async fn prompt_remove_user(pool: &SqlitePool) -> anyhow::Result<()> {
    let user = input::user_select(pool).await?;
    let Some(user) = user else {
        bail!("No user selected.")
    };
    let mut user = user.split_whitespace();
    User::delete_user(pool, user.next().unwrap(), user.next().unwrap()).await?;

    Ok(())
}
