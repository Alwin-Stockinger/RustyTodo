use serde::{Deserialize, Serialize};
use std::time::{SystemTime, Duration};

#[derive(Serialize, Deserialize)]
pub struct Task{
    name: String,
    due_time: SystemTime,
    work_time: std::time::Duration,
}

impl Task{
    pub fn new(name: String) -> Task{
        Task{
            name,
            due_time: SystemTime::now(),
            work_time: Duration::new(0, 0),
        }
    }
}
