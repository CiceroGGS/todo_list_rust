struct TodoItem {
    id: u64,
    title: String,
    completed: bool
}

pub struct TodoList {
    pub items: Vec<TodoItem>
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList { items: Vec::new() }
    }

    pub fn add_item(&mut self, title: String) -> () {
        let id: u64 = self.items.len() as u64 + 1u64;
        let new_item: TodoItem = TodoItem {
            id,
            title: title.clone(),
            completed: false
        };
        self.items.push(new_item);
        println!("Added item: {}", title);
    } 

    pub fn list_item(&self) -> () {
        if self.items.is_empty() {
            println!("No items in the todo list.");
            return;
        } else {
            println!("=== TO DO LIST ====");
            println!(" ");

            for item in &self.items {
                let status = if item.completed { "✓" } else { " " };
                println!("[{}] - {}: {}", status, item.id, item.title);
                println!("-------------------");
            }
        }
    }

    pub fn complete_item(&mut self, id: u64) -> () {
        if let Some(item) = self.items.iter_mut().find(|i|i.id == id) {
            item.completed = true;
            println!("Completed item: {}", item.title);
        } else {
            println!("Item with id {} not found.", id);
        }
    }
}