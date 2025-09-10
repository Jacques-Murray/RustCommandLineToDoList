use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

// Define the structure for a single To-Do item
struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    // Create a new To-Do list
    fn new() -> Result<Todo, io::Error> {
        let mut map = HashMap::new();
        let path_str = "db.txt";
        let path = Path::new(path_str);

        // Check if the database file exists
        if path.exists() {
            let file = File::open(path)?;
            let reader = BufReader::new(file);

            // Read the file line by line
            for line in reader.lines() {
                let line = line?;
                let mut parts = line.splitn(2, '\t');
                let key = parts.next().unwrap_or("").to_string();
                let value_str = parts.next().unwrap_or("false");
                let value = value_str.parse::<bool>().unwrap_or(false);
                if !key.is_empty() {
                    map.insert(key, value);
                }
            }
        }

        Ok(Todo { map })
    }

    // Insert a new item into the To-Do list
    fn insert(&mut self, key: &str) {
        self.map.insert(key.to_string(), true);
    }

    // Save the current state of the To-Do list to the file
    fn save(self) -> Result<(), io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record);
        }

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("db.txt")?;

        file.write_all(content.as_bytes())?;
        Ok(())
    }

    // Mark an item as complete
    fn complete(&mut self, key: &str) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => {
                *v = false;
                Some(())
            }
            None => None,
        }
    }

    // List all items
    fn list(&self) {
        println!("\n--- To-Do List ---");
        for (key, value) in &self.map {
            if *value {
                println!("[ ] {}", key);
            } else {
                println!("[X] {}", key);
            }
        }
        println!("------------------\n");
    }
}

fn main() {
  let args: Vec<String> = std::env::args().collect();

  if args.len() < 2 {
    println!("Usage: todo <action> [item]");
    println!("Actions: add, complete, list");
    return;
  }

  let action = &args[1];
  let item = if args.len()>2 { Some(&args[2]) } else { None };

  let mut todo = Todo::new().expect("Initialization of db failed");

  match action.as_str() {
    "add" => {
      if let Some(i)=item{
        todo.insert(i);
        match todo.save() {
          Ok(_) => println!("'{}' added.",i),
          Err(e) => println!("An error occurred: {}", e),
        }
      }else {
        println!("'add' action requires an item.");
      }
    }
    "complete" => {
      if let Some(i)=item{
        match todo.complete(i){
          None=>println!("'{}' is not present in the list",i),
          Some(_)=> match todo.save() {
            Ok(_)=>println!("'{}' completed",i),
            Err(e)=>println!("An error occurred: {}",e),
          },
        }
      } else {
        println!("'complete' action requires an item.");
      }
    }
    "list"=>{
      todo.list();
    }
    _ => {
      println!("Unknown action. Use 'add', 'complete', or 'list'.");
    }
  }
}
