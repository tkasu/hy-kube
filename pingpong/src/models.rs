use serde::Serialize;

#[derive(Clone, Debug, Serialize, sqlx::FromRow)]
pub struct PingStatus {
    pub ping_id: String,
    pub ping_count: i32,
}
