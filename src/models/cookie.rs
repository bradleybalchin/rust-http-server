use uuid::Uuid;
use chrono::{DateTime,Utc};

pub struct Session {
    id: Uuid,
    uuid: Uuid,
    created_at: DateTime<Utc>,
    expiry_date: DateTime<Utc>
}