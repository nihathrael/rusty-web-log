use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
	pub id: Uuid,
    pub mail: String,
    pub name: String,
    pub password: String,
}

impl ToJson for User {
    fn to_json(&self) -> Json {
        let mut m: BTreeMap<String, Json> = BTreeMap::new();
        m.insert("name".to_string(), self.name.to_json());
        m.insert("mail".to_string(), self.mail.to_json());
        m.insert("password".to_string(), self.password.to_json());
        m.to_json()
    }
}