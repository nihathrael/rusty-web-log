use iron::prelude::*;
use std::io::Read;
use url::form_urlencoded::parse;
use std::collections::BTreeMap;

pub fn get_form_data(req: &mut Request) -> BTreeMap<String, String> {
    let mut body = String::new();
    req.body.read_to_string(&mut body).ok().expect("Could not read body!");
    let form_data = parse(body.as_bytes());
    let mut data = BTreeMap::new();
    for (x,y) in form_data {
    	data.insert(x, y);
    }
    data
}
