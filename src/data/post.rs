use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;
use time::Tm;

pub struct Post {
	pub user: String,
	pub date: Tm,
	pub title: String,
	pub content: String
}

impl ToJson for Post {
    fn to_json(&self) -> Json {
        let mut m: BTreeMap<String, Json> = BTreeMap::new();
        m.insert("user".to_string(), self.user.to_json());
        m.insert("date".to_string(), format!("{}", self.date.asctime()).to_json());
        m.insert("title".to_string(), self.title.to_json());
        m.insert("content".to_string(), self.content.to_json());
        m.to_json()
    }
}
