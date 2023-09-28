use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Task {
    id: String,
    title: String,
    done: bool,
    date: String,
}

#[derive(Serialize, Deserialize)]
struct Database {
    tasks: Vec<Task>
}

fn list_tasks() {
    let db_file = fs::File::open("data.json").expect("could not read file");
    let database: Result<Database, serde_json::Error> = serde_json::from_reader(db_file);
    
    match database {
        Ok(db) => {
            for task in &db.tasks {
                println!("{}. || {} || {}", task.id, task.title, if task.done { "Completed" } else { "Not Completed" });
            }
        }
        Err(err) => {
            eprintln!("Error parsing JSON: {:?}", err);
        }
    }
}

fn add_task(title: &str) {
    let db_file = fs::File::open("data.json").expect("could not read file");
    let database: Result<Database, serde_json::Error> = serde_json::from_reader(db_file);
    
    let mut db = match database {
        Ok(db) => db,
        Err(err) => {
            eprintln!("Error parsing JSON: {:?}", err);
            Database { tasks: vec![] }
        }
    };

    let id = db.tasks.len() + 1;
    let task = Task {
        id: id.to_string(),
        title: title.to_string(),
        done: false,
        date: String::from(""),
    };

    db.tasks.push(task);

    let db_file = fs::File::create("data.json").expect("could not create file");
    let database = serde_json::to_writer_pretty(db_file, &db);
    
    match database {
        Ok(_) => {
            println!("Task added successfully");
        }
        Err(err) => {
            eprintln!("Error parsing JSON: {:?}", err);
        }
    }
}

fn main() {
    let args : Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        eprintln!("No command provided!");
        return;
    };

    let command = &args[1];
    if command == "ADD" {
        let title = &args[2];
        add_task(title);
    } else if command == "LIST" {
        list_tasks();
    } else {
        eprintln!("Invalid command");
    }

}
