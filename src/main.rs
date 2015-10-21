extern crate iron;
extern crate router;
extern crate handlebars_iron;
extern crate rustc_serialize;
extern crate mount;
extern crate staticfile;
extern crate time;
extern crate url;
extern crate uuid;
extern crate rusqlite;
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate persistent;

use handlebars_iron::{Template, HandlebarsEngine};
use iron::prelude::*;
use iron::status;
use iron::typemap::Key;
use router::Router;
use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;
use mount::Mount;
use staticfile::Static;
use std::path::Path;
use time::{now};
use uuid::Uuid;
use data::{User, Post};
use util::{ErrorReporter, get_form_data};
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::Pool;
use persistent::Read;

#[cfg(feature = "watch")]
use handlebars_iron::Watchable;
#[cfg(feature = "watch")]
use std::sync::Arc;


mod data;
mod util;

pub struct DatabasePool;
impl Key for DatabasePool { type Value = Pool<SqliteConnectionManager>; }

fn make_data() -> BTreeMap<String, Json> {
    let mut data = BTreeMap::new();
    
    let user = User {
    	id: Uuid::new_v4(),
        mail: "test@test.com".to_string(),
        name: "Firs Last".to_string(), 
        password: "test".to_string(),
    };
    data.insert("user".to_string(), user.to_json());
    data
}


#[allow(dead_code)]
fn main() {
    let mount = setup_routing();
    let mut chain = Chain::new(mount);
    let template_engine = get_templating_engine();
    chain.link_after(template_engine);
    chain.link_after(ErrorReporter);  
    
    let config = r2d2::Config::builder().
         error_handler(Box::new(r2d2::LoggingErrorHandler)).
         build();
    let manager = SqliteConnectionManager::new("/home/nihathrael/test.db").unwrap();

    let pool = Pool::new(config, manager).unwrap();

    create_tables(&pool);
	chain.link(Read::<DatabasePool>::both(pool));
    
    
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

fn index(req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    let mut data = make_data();
    let pool = req.get::<Read<DatabasePool>>().unwrap();
    let connection = pool.get().unwrap();
    let posts = Post::all(&connection);
    data.insert("posts".to_string(), posts.to_json());
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
    let pool = req.get::<Read<DatabasePool>>().unwrap();
    let connection = pool.get().unwrap();
    let post = Post {
    	user_id: 1,
    	title: form_data.get("title").unwrap().to_string(),
    	content:  form_data.get("content").unwrap().to_string(),
    	date: now().to_timespec(),
    };
    let statement = connection.execute("
	    INSERT INTO posts (user_id , title , content , date) VALUES (?, ?, ? ,?);
	", &[&post.user_id, &post.title, &post.content, &post.date]).unwrap();
	println!("{:?}", statement);
   
    let mut resp = Response::new();
    let data: BTreeMap<String, Json> = BTreeMap::new();
    resp.set_mut(Template::new("post", data)).set_mut(status::Ok);
    Ok(resp)
}

fn create_tables(pool: &Pool<SqliteConnectionManager>)  {
	let connection = pool.get().unwrap();
	connection.execute_batch("
	    CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT, password TEXT, mail TEXT);
	    CREATE TABLE IF NOT EXISTS posts (id INTEGER PRIMARY KEY AUTOINCREMENT, user_id INTEGER, title TEXT, content TEXT, date INTEGER);
	").unwrap();
}
