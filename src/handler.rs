use std::io::{stdin, Write, Read};
use std::fs::File;

use std::collections::HashMap;

use crate::project::{Project, task::Task};
use crate::review;
use crate::work;


fn print_options(){
    let options = vec!["project (new, list, review, work)", "quit", "help", "save", "load", "task(new, complete)", "review"];

    println!("\nAvailabe Options:");
    for x in options{
        println!("{}", x);
    }
}

pub(crate) static SAVE_DEFAULT_NAME: &str = "save.json";

pub struct Handler{
    projects: HashMap<String, Project>,
}

impl Handler{

    pub fn new() -> Handler{
        let mut handler = Handler{
            projects: HashMap::new(),
        };

        println!("Load default or migrate?");


        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();

        let mut commands = buffer.split_whitespace();

        let first = commands.next().unwrap();

        let rest: Vec<String> = commands.map(|x| String::from(x)).collect();

        match first{
            "yes" | "y" | "load" | "l" => handler.load(rest),
            "migrate" | "m" => {
                handler.projects = crate::migrate::migrate_save();
            },
            _ => {}
        }
        handler
    }

    pub fn handle(&mut self){
        loop{

            let mut buffer = String::new();
            stdin().read_line(&mut buffer).unwrap();

            let mut commands = buffer.split_whitespace();

            let first = commands.next().unwrap();

            let mut rest: Vec<String> = commands.map(|x| String::from(x)).collect();

            match first{
                "project" | "p" => self.handle_project(rest),
                "task" | "t" => self.handle_task(rest),
                "quit" | "q" => break,
                "help" | "h" => print_options(),
                "save" | "s" => self.save(rest),
                "load" | "l" => self.load(rest),
                "work" | "w" => self.work(rest.pop()),
                "review" | "r" => self.review_projects(),
                x => println!("{} is not an option, input help for availabe options", x),
            }
        }
        self.handle_quit();
    }

    fn handle_quit(&mut self){
        println!("Save?");

        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        let mut commands = buffer.split_whitespace();
        let first = commands.next().unwrap();
        let rest: Vec<String> = commands.map(|x| String::from(x)).collect();
        
        match first{
            "y" | "yes" => self.save(rest),
            "n" | "no" => return,
            _ => self.handle_quit(), 
        }
    }

    fn handle_project(&mut self, mut cmds: Vec<String>){
        cmds.reverse();

        //Optional Arguments
        let arg2 = cmds.pop(); //what to do
        let arg3 = cmds.pop(); //project name

        match arg2{
            Some(arg2) => {
                match arg2.as_str(){
                    "n" | "new" => self.new_project(arg3),
                    "w" | "work" => self.work(arg3),
                    "l" | "list" => self.list_projects(),
                    "r" | "review" => self.review_projects(),
                    x => println!("{} is not a valid argument", x),
                }
            }
            None => println!("No second argument given?!"),
        }
    }

    fn work(&mut self, project_name_op: Option<String>){
        match project_name_op{
            Some(name) => {
                match self.projects.get_mut(&name){
                    Some(project) => work::work_on_project(project),
                    None => println!("{} is not project", name),
                }
            }
            None => println!("You didn't specify which project to work on")
        }
    }

    fn new_project(&mut self, project_name_op: Option<String>){
        match project_name_op{
            Some(name) => {
                self.projects.insert(name.clone() ,Project::new(name));
            }
            None => println!("There was no name given for the new project"),
        }
    }

    fn handle_task(&mut self, mut cmds: Vec<String>){
        cmds.reverse();

        match cmds.pop(){
            Some(arg2) => {
                match arg2.as_str(){
                    "new" | "n" => {
                        match cmds.pop(){
                            Some(project_name) => {
                                match cmds.pop(){
                                    Some(task_name) => {
                                        match self.projects.get_mut(&project_name){
                                            Some(project) => project.add_task(Task::new(task_name)),
                                            None => println!("Invalid project name"),
                                        }
                                    }
                                    None => println!("No 4th argument given (task name)"),
                                }
                            }
                            None => println!("No 3rd argument given (projec name to which the task gets added)"),
                        }
                    }
                    "complete" | "c" => {
                        match cmds.pop(){
                            Some(project_name) => {
                                match cmds.pop(){
                                    Some(task_name) => {
                                        match self.projects.get_mut(&project_name){
                                            Some(project) => project.complete_task(task_name),
                                            None => println!("Invalid project name"),
                                        }
                                    }
                                    None => println!("No 4th argument given (task name)"),
                                }
                            }
                            None => println!("No 3rd argument given (projec name to which the task gets added)"),
                        }
                    }
                    x => println!("{} is not a valid 2nd argument", x),
                }
            }
            None => println!("Task What? 2nd argument is missing, e.g. new for new task"),
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
        for (_,project) in &self.projects {
            println!("{}", project.name);
        }
    }

    fn review_projects(&mut self){
        println!("\nProjects to review:");
        for (_, project) in self.projects.iter_mut() {
            if project.has_to_be_reviewed() {
                review::review_project(project);
            }
        }
    }
}
