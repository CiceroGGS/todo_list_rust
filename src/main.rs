use std::io;
mod todo_list;

fn main() {
    let mut todo_list = todo_list::todo::TodoList::new();

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
            },
            4 => {
                println!("Exiting the todo list application.");
                break;
            },
            _ => println!("Invalid option, please try again.")
        }
    }
}
