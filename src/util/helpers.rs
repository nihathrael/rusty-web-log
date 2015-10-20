use iron::prelude::*;
use std::io::Read;
use url::form_urlencoded::parse;

pub fn get_form_data(req: &mut Request) -> Vec<(String, String)> {
    let mut body = String::new();
    req.body.read_to_string(&mut body).ok().expect("Could not read body!");
    parse(body.as_bytes())
}
