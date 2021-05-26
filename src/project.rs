use serde::{Deserialize, Serialize};

use std::time::SystemTime;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize)]
pub struct Project{
    pub name: String,
    review_time: SystemTime,
    pub tasks: HashSet<String>,
    completed_tasks: HashSet<String>,
}

impl Project{
    pub fn new(name: String) -> Project{
        Project{
            name,
            review_time: SystemTime::now(),
            tasks: HashSet::new(),
            completed_tasks: HashSet::new(),
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

    pub fn add_task(&mut self, task: String){
        self.tasks.insert(task);
    }

    pub fn complete_task(&mut self, task: String){
        if self.tasks.remove(&task){
            self.completed_tasks.insert(task);
        } else {
            println!("No task named {} in the project", task);
        }
    }

    pub fn set_next_review_days(&mut self, days: u64){
        self.set_next_review(SystemTime::now() + std::time::Duration::from_secs(60*60*24*days)); 
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

