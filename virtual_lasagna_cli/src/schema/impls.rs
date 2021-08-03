use crate::schema;

impl schema::Post {
    pub fn new(content: impl ToString, num: u32) -> Self {
        Self {
            content: content.to_string(),
            num,
            created: chrono::offset::Local::now(),
            updated: None,
            is_deleted: None,
        }
    }
}
