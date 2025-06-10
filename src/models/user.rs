use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct User {
    pub user_id: i64,
    pub first_name: String,
    pub last_name: String,
}
