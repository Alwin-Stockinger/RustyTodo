use serde::{Deserialize, Serialize};
use std::time::{SystemTime, Duration, UNIX_EPOCH};
use chrono;
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task{
    pub name: String,
    due_time: SystemTime,
    work_time: std::time::Duration,
}

impl Task{
    pub fn new(name: String, due_time: SystemTime) -> Task{
        Task{
            name,
            due_time,
            work_time: Duration::new(0, 0),
        }
    }

    pub fn add_time_since(&mut self, start: SystemTime){
        self.work_time += SystemTime::now().duration_since(start).unwrap();
    }

    pub fn get_work_time(& self) -> std::time::Duration{
        self.work_time.clone()
    }

    pub fn string_due_date(&self) -> String{
        let secs_since_unix =self.due_time.duration_since(UNIX_EPOCH).expect("Could not calculate duration since epoch").as_secs();
        let date = chrono::NaiveDateTime::from_timestamp(secs_since_unix as i64, 0);
        format!("{}", date)
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
