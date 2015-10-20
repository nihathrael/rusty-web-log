use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;
use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Post {
    pub user_id: Uuid,
    pub date: DateTime<UTC>,
    pub title: String,
    pub content: String,
}

impl ToJson for Post {
    fn to_json(&self) -> Json {
        let mut m: BTreeMap<String, Json> = BTreeMap::new();
        m.insert("user".to_string(), self.user_id.to_simple_string().to_json());
        m.insert("date".to_string(),
                 format!("{}", self.date.to_string()).to_json());
        m.insert("title".to_string(), self.title.to_json());
        m.insert("content".to_string(), self.content.to_json());
        m.to_json()
    }
}