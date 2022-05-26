use super::schema::ping_status;
use serde::Serialize;

#[derive(Clone, Debug, Queryable, Insertable, Serialize, sqlx::FromRow)]
#[table_name = "ping_status"]
pub struct PingStatus {
    pub ping_id: String,
    pub ping_count: i32,
}
