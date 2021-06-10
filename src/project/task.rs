use serde::{Deserialize, Serialize};
use std::time::{SystemTime, Duration};
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task{
    pub name: String,
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

impl PartialEq for Task{
    fn eq(&self, other: &Self) -> bool{
        self.name == other.name
    }
}

impl Eq for Task{
}

impl Hash for Task{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
