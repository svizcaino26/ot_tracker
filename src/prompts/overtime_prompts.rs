use crate::Overtime;
use crate::SqlitePool;
use crate::input;
use time::Duration;

pub async fn prompt_add_overtime(pool: &SqlitePool) -> anyhow::Result<()> {
    let user = input::user_select(pool).await?;
    let mut user = user.split_whitespace();
    let description = input::description_input("Description: ")?;

    let ot = Some(
        Overtime::start_tracking(
            pool,
            user.next().unwrap(),
            user.next().unwrap(),
            &description,
        )
        .await?,
    );

    if let Ok(true) = input::confirm("Stop recording?") {
        if let Some(mut ot) = ot {
            ot.end_tracking(pool).await?;
        }
    }

    Ok(())
}

pub async fn prompt_get_overtime(pool: &SqlitePool) -> anyhow::Result<Duration> {
    let total_overtime = Overtime::calculate_total_overtime(pool).await?;
    Ok(total_overtime)
}
