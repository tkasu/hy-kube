use serde::Serialize;
use super::schema::ping_status;

#[derive(Clone, Debug, Queryable, Insertable, Serialize)]
#[table_name="ping_status"]
pub struct PingStatus {
    pub ping_id: String,
    pub ping_count: i32
}
