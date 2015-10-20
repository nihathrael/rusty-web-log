extern crate iron;
extern crate router;
extern crate handlebars_iron;
extern crate rustc_serialize;
extern crate mount;
extern crate staticfile;
extern crate time;
extern crate url;
extern crate chrono;
extern crate uuid;
extern crate sqlite;

use handlebars_iron::{Template, HandlebarsEngine};
use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;
use mount::Mount;
use staticfile::Static;
use std::path::Path;
use time::now;
use chrono::offset::utc::UTC;
use uuid::Uuid;
use data::{User, Post};
use util::{ErrorReporter, get_form_data};
use sqlite::Connection;
use sqlite::Value;

#[cfg(feature = "watch")]
use handlebars_iron::Watchable;
#[cfg(feature = "watch")]
use std::sync::Arc;


mod data;
mod util;

fn make_data() -> BTreeMap<String, Json> {
    let mut data = BTreeMap::new();
    
    let user = User {
    	id: Uuid::new_v4(),
        mail: "test@test.com".to_string(),
        name: "Firs Last".to_string(), 
        password: "test".to_string(),
    };
    data.insert("user".to_string(), user.to_json());

    let posts = vec![Post {
                         user_id: user.id,
                         date: UTC::now(),
                         content: "This is my first Post. <b>a Test!</b>".to_string(),
                         title: "First Blogpost".to_string(),
                     },
                     Post {
                         user_id: user.id,
                         date: UTC::now(),
                         content: "test".to_string(),
                         title: "Second Blogpost".to_string(),
                     }];

    data.insert("posts".to_string(), posts.to_json());
    data
}


#[allow(dead_code)]
fn main() {
    let mount = setup_routing();
    let mut chain = Chain::new(mount);
    let template_engine = get_templating_engine();
    chain.link_after(template_engine);
    chain.link_after(ErrorReporter);  
    
//    let connection = sqlite::open(":memory:").unwrap();
    let connection = sqlite::open("/home/nihathrael/test.db").unwrap();
    create_tables(&connection);
    
    Iron::new(chain).http("localhost:3000").unwrap();
}

#[cfg(feature = "watch")]
fn get_templating_engine() -> Arc<HandlebarsEngine> {
    let template_engine = Arc::new(HandlebarsEngine::new("templates/", ".hbs"));
    template_engine.watch();
    template_engine
}

#[cfg(not(feature = "watch"))]
fn get_templating_engine() -> HandlebarsEngine {
    HandlebarsEngine::new("templates/", ".hbs")
}

fn setup_routing() -> Mount {
    let mut router = Router::new();
    router.get("/", index);
    router.get("/post/", post);
    router.post("/post/save", save_post);
    let mut mount = Mount::new();
    mount.mount("/", router)
         .mount("/css/", Static::new(Path::new("resources/css")))
         .mount("/images/", Static::new(Path::new("resources/images")))
         .mount("/js/", Static::new(Path::new("resources/js")))
         .mount("/fonts/", Static::new(Path::new("resources/fonts")));
    mount
}

fn index(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    let data = make_data();
    resp.set_mut(Template::new("index", data)).set_mut(status::Ok);
    Ok(resp)
}

fn post(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    let data: BTreeMap<String, Json> = BTreeMap::new();
    resp.set_mut(Template::new("post", data)).set_mut(status::Ok);
    Ok(resp)
}

fn save_post(req: &mut Request) -> IronResult<Response> {
    let form_data = get_form_data(req);
    let connection = sqlite::open("/home/nihathrael/test.db").unwrap();
    let mut statement = connection.prepare("
	    INSERT INTO posts (user_id , title , content , date) VALUES (?, ?, ? ,?);
	").unwrap();
    // TODO (TK): Figure out how to find the user from the session (that does not exist yet...)
    statement.bind(1, 1).unwrap();
    statement.bind(2, Value::String(form_data.get("title").unwrap().to_string())).unwrap();
    statement.bind(3, Value::String(form_data.get("content").unwrap().to_string())).unwrap();
    // TODO (TK): Save date
    statement.bind(4, 123).unwrap();
    statement.next().unwrap();
   
    let mut resp = Response::new();
    let data: BTreeMap<String, Json> = BTreeMap::new();
    resp.set_mut(Template::new("post", data)).set_mut(status::Ok);
    Ok(resp)
}

fn create_tables(connection: &Connection) {
	connection.execute("
	    CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT, password TEXT, mail TEXT);
	    CREATE TABLE IF NOT EXISTS posts (id INTEGER PRIMARY KEY AUTOINCREMENT, user_id INTEGER, title TEXT, content TEXT, date INTEGER);
	").unwrap();
}
