//use std::fs::File;
//use std::io::Write;
//use std::io::prelude::*;

//use std::time::SystemTime;

mod project;
mod handler;
mod review;
mod work;

fn main() {
    let mut handler = handler::Handler::new();

    handler.handle();
}
