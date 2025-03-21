use crate::pb::User;
use prost_types::Timestamp;
use std::time::SystemTime;

impl User {
    pub fn new(id: u64, name: &str, email: &str) -> Self {
        let now = SystemTime::now();
        User {
            id,
            name: name.to_string(),
            email: email.to_string(),
            created_at: Some(Timestamp::from(now)),
        }
    }
}
