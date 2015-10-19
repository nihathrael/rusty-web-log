extern crate iron;
extern crate router;
extern crate handlebars_iron;
extern crate rustc_serialize;
extern crate mount;
extern crate staticfile;
extern crate time;
extern crate url;

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

use data::{User, Post};
use util::{ErrorReporter, get_form_data};

#[cfg(feature = "watch")]
use handlebars_iron::Watchable;
#[cfg(feature = "watch")]
use std::sync::Arc;


mod data;
mod util;

fn make_data () -> BTreeMap<String, Json> {
    let mut data = BTreeMap::new();

  let posts = vec![ 
    	Post { 
	    	user: "test1@test.com".to_string(), 
	    	date: now(), 
	    	content: "This is my first Post. <b>a Test!</b>".to_string(),
	    	title: "First Blogpost".to_string()
    	}, 
    	Post { 
	    	user: "test2@test.com".to_string(), 
	    	date: now(), 
	    	content: "test".to_string(),
	    	title: "Second Blogpost".to_string() 
    	}];

    let user =  User { 
	    	mail: "test@test.com".to_string(), 
	    	name: "Firs Last".to_string(), 
	    	password: "test".to_string() 
    	};
 	data.insert("user".to_string(), user.to_json());
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
    mount.
    	mount("/", router).
    	mount("/css/", Static::new(Path::new("resources/css"))).
    	mount("/images/", Static::new(Path::new("resources/images"))).
    	mount("/js/", Static::new(Path::new("resources/js"))).
    	mount("/fonts/", Static::new(Path::new("resources/fonts")));
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
	println!("{:?}", req.headers);
	println!("{:?}", form_data);
    let mut resp = Response::new();
    let data: BTreeMap<String, Json> = BTreeMap::new();
    resp.set_mut(Template::new("post", data)).set_mut(status::Ok);
    Ok(resp)
}