use std::fs;
use serde::{Deserialize, Serialize};
use clap::Parser;

#[derive(Parser, Serialize, Deserialize)]
struct Task {
    id: String,
    title: String,
    done: bool,
    date: String,
}

#[derive(Deserialize)]
struct Database {
    tasks: Vec<Task>
}

fn main() {
    let db_file = fs::File::open("data.json").expect("could not read file");
    let database: Result<Database, serde_json::Error> = serde_json::from_reader(db_file);
    
    match database {
        Ok(db) => {
            for task in &db.tasks {
                println!("{}. || {} || {}", task.id, task.title, if task.done { "Completed" } else { "Not Completed" });
                println!();
            }
        }
        Err(err) => {
            eprintln!("Error parsing JSON: {:?}", err);
        }
    }
}
