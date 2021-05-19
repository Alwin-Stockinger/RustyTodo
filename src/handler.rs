use std::io::stdin;

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

    fn list_projects(&self){
        println!("\nProjects:");
        for project in &self.projects {
            println!("{}", project.name);
        }
    }
}
