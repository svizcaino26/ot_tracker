use crate::OffsetDateTime;
use crate::Rfc3339;
use time::Duration;

pub fn get_current_time() -> anyhow::Result<String> {
    let now = OffsetDateTime::now_local()
        .map_err(|e| anyhow::anyhow!("Failed to get local time: {}", e))?
        .format(&Rfc3339)
        .map_err(|e| anyhow::anyhow!("Failed to format time: {}", e))?;
    Ok(now)
}

pub fn pause() {
    use std::io::{self, Write};
    print!("Press Enter to continue...");
    io::stdout().flush().unwrap();
    let _ = io::stdin().read_line(&mut String::new());
}

pub fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.whole_seconds();

    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    format!("{}h {}m {}s", hours, minutes, seconds)
}
