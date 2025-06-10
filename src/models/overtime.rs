use crate::FromRow;
use crate::OffsetDateTime;

#[derive(Debug, FromRow)]
pub struct Overtime {
    pub ot_id: Option<i64>,
    pub user_id: Option<i64>,
    pub start_time: Option<OffsetDateTime>,
    pub end_time: Option<OffsetDateTime>,
    pub description: Option<String>,
}
