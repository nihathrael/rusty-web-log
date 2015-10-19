use iron::AfterMiddleware;
use iron::prelude::*;
use std::error::Error;

pub struct ErrorReporter;

impl AfterMiddleware for ErrorReporter {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        println!("Templating error: {}", err.description());
        Err(err)
    }
}