use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;
use time::Timespec;
use time::at;
use rusqlite::SqliteConnection;

#[derive(Debug, Clone)]
pub struct Post {
    pub user_id: i32,
    pub date: Timespec,
    pub title: String,
    pub content: String,
}

impl Post {
	pub fn all(connection: &SqliteConnection) -> Vec<Post> {
		let mut statement = connection.prepare("SELECT user_id, title, content, date FROM posts").unwrap();
		let post_iter = statement.query_map(&[], |row| {
			let user_id: i32 = row.get(0);
			let title: String = row.get(1);
			let content: String = row.get(2);
			let date: Timespec = row.get(3);
			
       		Post {
       			user_id: user_id,
       			date: date,
       			title: title,
       			content: content
       		}
    	}).unwrap();
		let mut posts = Vec::new();
		for post in post_iter {
			posts.push(post.unwrap());
		}
		posts
	}
}

impl ToJson for Post {
    fn to_json(&self) -> Json {
        let mut m: BTreeMap<String, Json> = BTreeMap::new();
        m.insert("user".to_string(), self.user_id.to_json());
        m.insert("date".to_string(),
                 format!("{}", at(self.date).rfc822()).to_json());
        m.insert("title".to_string(), self.title.to_json());
        m.insert("content".to_string(), self.content.to_json());
        m.to_json()
    }
}

