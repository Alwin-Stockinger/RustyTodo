use std::io::stdin;


use crate::project::Project;


pub fn review_project(project: &mut Project){
    println!("\n");
    println!("Project name: {}", project.name);
    println!("Incomplete Tasks: {:?}", project.tasks);

    loop{
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();

        let mut commands = buffer.split_whitespace();

        let first = commands.next().unwrap();

        let rest: Vec<String> = commands.map(|x| String::from(x)).collect();

        match first{
            "finish" => {
                handle_finish(project);
                break;
            }
            "task" => handle_task(project, rest),
            "help" => print_options(),
            x => println!("{} is not an option, input help for availabe options", x),
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


fn handle_task(project:&mut Project, mut cmds: Vec<String>){
    cmds.reverse();

    match cmds.pop(){
        Some(arg) => {
            match arg.as_str() {
                "new" => {
                    match cmds.pop(){
                        Some(task_name) => {
                            project.add_task(task_name)
                        }
                        None => println!("No 3rd argument given (task name)"),
                    }
                }
                "complete" => {
                    match cmds.pop(){
                        Some(task_name) => {
                            project.complete_task(task_name)
                        }
                        None => println!("No 3rd argument given (task name)"),
                    }
                }
                x => println!("{} is not a valid 2nd argument", x),
            }
        }
        None => println!("Missing 2nd argument"),
    }
}

fn handle_finish(project:&mut Project){
    println!("\nNext reivew in how many days?");

    loop{
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();

        match buffer.trim().parse::<u64>(){
            Ok(x) =>{
                project.set_next_review_days(x);
                break;
            }
            _ => println!("{} is not a valid number", buffer),
        }
    }
}
