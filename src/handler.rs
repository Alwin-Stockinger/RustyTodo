use std::io::{stdin, Write, Read};
use std::fs::File;

use std::collections::HashSet;

use crate::project::Project;

pub enum Action{
    Continue,
    Quit,
}

fn print_options(){
    let options = vec!["project", "quit", "help"];

    println!("\nAvailabe Options:");
    for x in options{
        println!("{}", x);
    }
}

static SAVE_DEFAULT_NAME: &str = "save.json";

pub struct Handler{
    projects: HashSet<Project>,
}

impl Handler{

    pub fn new() -> Handler{
        Handler{
            projects: HashSet::new(),
        }
    }

    pub fn handle(&mut self){
        loop{

            let mut buffer = String::new();
            stdin().read_line(&mut buffer).unwrap();

            let mut commands = buffer.split_whitespace();

            let first = commands.next().unwrap();

            let rest: Vec<String> = commands.map(|x| String::from(x)).collect();

            match first{
                "project" => self.handle_project(rest),
                "quit" => break,
                "q" => break,
                "help" => print_options(),
                "save" => self.save(rest),
                "load" => self.load(rest),
                x => println!("{} is not an option, input help for availabe options", x),
            }
        }
    }

    fn handle_project(&mut self, mut cmds: Vec<String>){
        cmds.reverse();

        match cmds.pop(){
            Some(arg2) => {
                match arg2.as_str(){
                    "new" => {
                        match cmds.pop(){
                            Some(name) => {
                                self.projects.insert( Project::new(name));
                            }
                            None => println!("There was no name given for the new project"),
                        }
                    }
                    "list" => self.list_projects(),
                    x => println!("{} is not a valid argument", x),
                }
            }
            None => println!("No second argument given?!"),
        }
    }

    fn save(&self, mut cmds: Vec<String>){
        let save_name = match cmds.pop(){
            Some(name) => name,
            None => String::from(SAVE_DEFAULT_NAME), 
        };

        let json_string = serde_json::to_string(&self.projects).unwrap();

        let mut out = File::create(save_name).unwrap();
        write!(out, "{}", json_string).unwrap();
    }

    fn load(&mut self, mut cmds: Vec<String>){
        let load_name = match cmds.pop(){
            Some(name) => name,
            None => String::from(SAVE_DEFAULT_NAME),
        };

        let mut json_string = String::new();

        let mut file = File::open(load_name).unwrap();

        file.read_to_string(&mut json_string).unwrap();

        self.projects = serde_json::from_str(&json_string).unwrap();

    }

    fn list_projects(&self){
        println!("\nProjects:");
        for project in &self.projects {
            println!("{}", project.name);
        }
    }
}
