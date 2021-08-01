use serde::{Deserialize, Serialize};

use std::time::{SystemTime, Duration};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub mod task;
use task::Task;

#[derive(Serialize, Deserialize)]
pub struct Project{
    pub name: String,
    pub(crate) review_time: SystemTime,
    pub tasks: HashMap<String, Task>,
    pub(crate) completed_tasks: HashMap<String, Task>,
    pub(crate) work_time: std::time::Duration,
}

impl Project{
    pub fn new(name: String) -> Project{
        Project{
            name,
            review_time: SystemTime::now(),
            tasks: HashMap::new(),
            completed_tasks: HashMap::new(),
            work_time: Duration::new(0, 0), 
        }
    }
    pub fn has_to_be_reviewed(&self) -> bool{
        if self.review_time < SystemTime::now(){
            true
        } else {
            false
        }
    }
    pub fn set_next_review(&mut self, time: SystemTime){
        self.review_time = time;
    }

    pub fn add_task(&mut self, task: Task){
        self.tasks.insert(task.name.clone(), task);
    }

    pub fn complete_task(&mut self, task_name: String){
        if let Some(task) = self.tasks.remove(&task_name){
            self.completed_tasks.insert(task_name, task);
        } else {
            eprintln!("No task named {} in the project", task_name);
        }
    }

    pub fn set_next_review_days(&mut self, days: u64){
        self.set_next_review(SystemTime::now() + Duration::from_secs(60*60*24*days)); 
    }

    pub fn add_work_time(&mut self, time: Duration){
        self.work_time += time;
    }

    pub fn get_work_time(&self) -> Duration{
        self.work_time.clone()
    }
}

impl PartialEq for Project{
    fn eq(&self, other: &Self) -> bool{
        self.name == other.name
    }
}

impl Eq for Project{
}

impl Hash for Project {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Default for Project{
    fn default() -> Self {
        Project::new("".to_string())
    }
}
