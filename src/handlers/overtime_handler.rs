use crate::OffsetDateTime;
use crate::SqlitePool;
use crate::User;
use crate::get_current_time;
pub use crate::models::overtime::Overtime;

impl Overtime {
    pub async fn start_tracking(
        pool: &SqlitePool,
        first_name: &str,
        last_name: &str,
        description: &str,
    ) -> anyhow::Result<Overtime> {
        let user_id = User::get_user(pool, first_name, last_name).await?.user_id;

        let now = get_current_time()?;

        let ot = sqlx::query_as!(
            Overtime,
            r#"
        INSERT INTO overtime (user_id, start_time, description)
        VALUES(?1, ?2, ?3)
        RETURNING ot_id, user_id, start_time as "start_time: OffsetDateTime",
        end_time as "end_time: OffsetDateTime", description
        "#,
            user_id,
            now,
            description
        )
        .fetch_one(pool)
        .await?;

        Ok(ot)
    }

    pub async fn end_tracking(&mut self, pool: &SqlitePool) -> anyhow::Result<()> {
        let now = get_current_time()?;

        sqlx::query!(
            r#"
        UPDATE overtime
        SET end_time = ?1
        WHERE ot_id = ?2
        "#,
            now,
            self.ot_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
