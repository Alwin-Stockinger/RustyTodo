use std::fs::File;
use std::io::{Read, stdin};

mod old;

use std::collections::HashMap;
use crate::project::Project;
use crate::project::task::Task;
use std::time::UNIX_EPOCH;

pub fn migrate_save() -> (HashMap<String, Project>, HashMap<String, Project>) {
    println!("From which version?(0.1: Tasks are strings, 0.2 meaningful Tasks)");

    loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();

        let mut commands = buffer.split_whitespace();

        let first = commands.next();

        match first {
            Some("0.1") => return dot_one_migration(),
            Some("0.2") => return dot_two_migration(),
            _ => {
                println!("that version doesnt exist");
                continue;
            }
        }
    }
}

pub fn dot_two_migration() -> (HashMap<String, Project>, HashMap<String, Project>) {

    let load_name = crate::handler::SAVE_DEFAULT_NAME;

    let mut json_string = String::new();

    let mut file = File::open(load_name).unwrap();

    file.read_to_string(&mut json_string).unwrap();

    let projects: HashMap<String, Project> = serde_json::from_str(&json_string).unwrap();

    return (projects, HashMap::new());
}

pub fn dot_one_migration() -> (HashMap<String, Project>, HashMap<String, Project>) {

    let load_name = crate::handler::SAVE_DEFAULT_NAME;

    let mut json_string = String::new();

    let mut file = File::open(load_name).unwrap();

    file.read_to_string(&mut json_string).unwrap();

    let mut projects: HashMap<String, old::project1::Project> = serde_json::from_str(&json_string).unwrap();


    let active_projects = projects.drain().map(|(project_name, mut project)|{
        let migrated_project = Project{
            name: project.name,
            review_time: project.review_time,
            tasks: project.tasks.drain()
                .map(|task_name| (task_name.clone(), Task::new(task_name, UNIX_EPOCH)))
                .collect(),
            completed_tasks: project.completed_tasks.drain()
                .map(|task_name| (task_name.clone(), Task::new(task_name, UNIX_EPOCH)))
                .collect(),
            work_time: project.work_time,
        };
        (project_name, migrated_project)
    }).collect();

    return (active_projects, HashMap::new());
}
