pub use crate::models::user::User;
use anyhow::Context;
use sqlx::sqlite::SqlitePool;

impl User {
    pub async fn create_user(
        pool: &SqlitePool,
        first_name: &str,
        last_name: &str,
    ) -> anyhow::Result<Self> {
        if Self::exists(pool, first_name, last_name).await? {
            return Err(anyhow::anyhow!("User already exists"));
        }

        let user = sqlx::query_as!(
            User,
            r#"
        INSERT INTO users (first_name, last_name)
        VALUES(?1, ?2)
        RETURNING user_id, first_name, last_name
        "#,
            first_name,
            last_name
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user(
        pool: &SqlitePool,
        first_name: &str,
        last_name: &str,
    ) -> anyhow::Result<Self> {
        let user = sqlx::query_as!(
            User,
            r#"
        SELECT user_id, first_name, last_name
        FROM users
        WHERE first_name = ?1 AND last_name = ?2
        LIMIT 1 "#,
            first_name,
            last_name
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn exists(
        pool: &SqlitePool,
        first_name: &str,
        last_name: &str,
    ) -> anyhow::Result<bool> {
        let exists = sqlx::query_scalar!(
            r#"
        SELECT EXISTS(
        SELECT 1 FROM users
        WHERE first_name = ?1 AND last_name = ?2
        )
        "#,
            first_name,
            last_name
        )
        .fetch_one(pool)
        .await?
            > 0;

        Ok(exists)
    }

    pub async fn list_users(pool: &SqlitePool) -> anyhow::Result<Vec<User>> {
        let users: Vec<User> = sqlx::query_as!(User, r#"SELECT * FROM users ORDER BY user_id"#)
            .fetch_all(pool)
            .await?;
        Ok(users)
    }

    pub async fn delete_user(
        pool: &SqlitePool,
        first_name: &str,
        last_name: &str,
    ) -> anyhow::Result<()> {
        let user = Self::get_user(pool, first_name, last_name).await?;
        let query = sqlx::query_as!(
            User,
            r#"
            DELETE FROM users
            WHERE user_id = ?1
            "#,
            user.user_id
        )
        .execute(pool)
        .await
        .context("failed to delete user")?;

        if query.rows_affected() == 0 {
            anyhow::bail!("No user found with name {} {}", first_name, last_name);
        }

        Ok(())
    }
}
