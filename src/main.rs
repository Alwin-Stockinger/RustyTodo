use std::fs::File;
use std::io::Write;
use std::io::prelude::*;

use std::time::SystemTime;

mod project;
use project::Project;


fn main(){

    let mut idp = Project::new("IDP".to_owned());

    println!("{}", idp.has_to_be_reviewed());

    let now = SystemTime::now();
    let week = std::time::Duration::from_secs(60*60*24*7);
    let next = now + week;
    idp.set_next_review(next);
    idp.add_task("dfinity tutorial".to_owned());

    println!("{}", idp.has_to_be_reviewed());

    /*
    let string = serde_json::to_string(&idp).unwrap();

//    println!("{}",string);

    let mut out = File::create("out.json").unwrap();
    write!(out, "{}", string).unwrap();


    let mut written = String::new();
    let mut input = File::open("out.json").unwrap();
    input.read_to_string(&mut written).unwrap();


    println!("{}", written);*/

}
