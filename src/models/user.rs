use uuid::Uuid;

pub struct User {
    id: Uuid,
    username: String,
    password_hash: String,
    is_admin: bool,
}