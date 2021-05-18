use serde::{Deserialize, Serialize};

use std::time::SystemTime;
use std::collections::HashSet;

#[derive(Serialize, Deserialize)]
pub struct Project{
    name: String,
    review_time: SystemTime,
    tasks: HashSet<String>,
}

impl Project{
    pub fn new(name: String) -> Project{
        Project{
            name,
            review_time: SystemTime::now(),
            tasks: HashSet::new(),
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
}

