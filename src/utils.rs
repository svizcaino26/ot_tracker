use crate::OffsetDateTime;
use crate::Rfc3339;

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
