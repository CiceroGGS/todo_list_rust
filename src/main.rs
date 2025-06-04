use std::io;

struct TodoItem {
    id: u64,
    title: String,
    completed: bool
}

struct TodoList {
    items: Vec<TodoItem>
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { items: Vec::new() }
    }

    fn add_item(&mut self, title: String) -> () {
        let id: u64 = self.items.len() as u64 + 1u64;
        let new_item: TodoItem = TodoItem {
            id,
            title: title.clone(),
            completed: false
        };
        self.items.push(new_item);
        println!("Added item: {}", title);
    } 

    fn list_item(&self) -> () {
        if self.items.is_empty() {
            println!("No items in the todo list.");
            return;
        } else {
            println!("=== TO DO LIST ====");

            for item in &self.items {
                let status = if item.completed { "✓" } else { "[ ]" };
                println!("[{}] {}: {}", status, item.id, item.title);
            }
        }
    }

    fn complete_item(&mut self, id: u64) -> () {
        if let Some(item) = self.items.iter_mut().find(|i|i.id == id) {
            item.completed = true;
            println!("Completed item: {}", item.title);
        } else {
            println!("Item with id {} not found.", id);
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    loop {
        println!("1. Add item");
        println!("2. List items");
        println!("3. Complete item");
        println!("4. Exit");
        println!("Choose an option:");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match choice {
            1 => {
                println!("Enter item title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Failed to read line");
                todo_list.add_item(title.trim().to_string());
            },
            2 => {
                println!("Listing items:");
                todo_list.list_item();
            },
            3 => {
                println!("Enter item id to complete:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id: u64 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                todo_list.complete_item(id);
            }
            _ => {}
        }
    }
}
