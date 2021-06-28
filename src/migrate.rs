use std::fs::File;
use std::io::Read;

mod old;

use std::collections::HashMap;
use crate::project::Project;
use crate::project::task::Task;

pub fn migrate_save() -> HashMap<String, Project>{

    let load_name = crate::handler::SAVE_DEFAULT_NAME;

    let mut json_string = String::new();

    let mut file = File::open(load_name).unwrap();

    file.read_to_string(&mut json_string).unwrap();

    let mut projects: HashMap<String, old::project1::Project> = serde_json::from_str(&json_string).unwrap();


    return projects.drain().map(|(project_name, mut project)|{
        let migrated_project = Project{
            name: project.name,
            review_time: project.review_time,
            tasks: project.tasks.drain()
                .map(|task_name| (task_name.clone(), Task::new(task_name)))
                .collect(),
            completed_tasks: project.completed_tasks.drain()
                .map(|task_name| (task_name.clone(), Task::new(task_name)))
                .collect(),
            work_time: project.work_time,
        };
        (project_name, migrated_project)
    }).collect();
}
