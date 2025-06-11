use crate::OffsetDateTime;
use crate::SqlitePool;
use crate::User;
pub use crate::models::overtime::Overtime;
use crate::utils;
use time::Duration;
use time::format_description::well_known::Rfc3339;

impl Overtime {
    pub async fn start_tracking(
        pool: &SqlitePool,
        first_name: &str,
        last_name: &str,
        description: &str,
    ) -> anyhow::Result<Overtime> {
        let user_id = User::get_user(pool, first_name, last_name).await?.user_id;

        let now = utils::get_current_time()?;

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
        let now = utils::get_current_time()?;

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

    pub async fn calculate_total_overtime(pool: &SqlitePool) -> anyhow::Result<Duration> {
        let records = sqlx::query!(
            r#"
        SELECT start_time, end_time FROM overtime
        "#,
        )
        .fetch_all(pool)
        .await?;

        let total_overtime = records
            .iter()
            .filter_map(|record| {
                let end_dt = OffsetDateTime::parse(record.end_time.as_ref()?, &Rfc3339).ok()?;
                let start_dt = OffsetDateTime::parse(record.start_time.as_ref()?, &Rfc3339).ok()?;
                Some(end_dt - start_dt)
            })
            .fold(Duration::ZERO, |acc, dur| acc + dur);

        Ok(total_overtime)
    }
}

// fn get_overtime_total<T>(records: Vec<T>) -> anyhow::Result<Duration> {
//     let total_overtime = records
//         .iter()
//         .filter_map(|record| {
//             let end_dt = OffsetDateTime::parse(record.end_time.as_ref()?, &Rfc3339).ok()?;
//             let start_dt = OffsetDateTime::parse(record.start_time.as_ref()?, &Rfc3339).ok()?;
//             Some(end_dt - start_dt)
//         })
//         .fold(Duration::ZERO, |acc, dur| acc + dur);
//
//     Ok(total_overtime)
// }
