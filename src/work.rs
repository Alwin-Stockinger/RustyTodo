use std::io::stdin;


use crate::project::Project;



pub fn work_on_project(project:&mut Project){
    println!("\nWork on project {} started", project.name);

    let start_time = std::time::SystemTime::now();

    loop{
        println!("\nTasks:");
        for task in &project.tasks{
            println!("{}", task);
        }

        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();

        let mut cmds = buffer.split_whitespace();

        let first = cmds.next().unwrap();

        let mut rest: Vec<String> = cmds.map(|x| String::from(x)).collect();
        rest.reverse();

        match first{
            "task" => work_task(project, rest.pop()),
            "finish" => {
                project.add_work_time(start_time.elapsed().unwrap());
                break;
            }
            _  => print_options(),
        }
    }

}

fn print_options(){
    let options = vec!["task", "finish", "help"];

    println!("\nAvailabe Options:");
    for x in options{
        println!("{}", x);
    }
}


fn print_task_options(){
    let options = vec!["complete", "shelf", "help"];

    println!("\nAvailabe Options:");
    for x in options{
        println!("{}", x);
    }
}


fn work_task(project: &mut Project, task_opt: Option<String>){
    if None == task_opt{
        println!(" No task specified");
        return;
    }
    let task = task_opt.unwrap();
    if None == project.tasks.get(&task){
        println!("{} is not a task in project {}", task, project.name);
        return;
    }

    loop{
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();

        let cmd = buffer.trim();

        match cmd{
            "complete" => {
                project.complete_task(task);
                return;
            }
            "shelf" => return,
            _ => print_task_options(),
        }
    }
}
