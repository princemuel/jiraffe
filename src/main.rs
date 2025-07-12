use std::rc::Rc;

use jiraffe::database::JiraDatabase;
use jiraffe::io::{pause, read_line};
use jiraffe::navigator::Navigator;

fn main() {
    let db = Rc::new(JiraDatabase::new("./data/db.json".to_string()));
    let mut navigator = Navigator::new(Rc::clone(&db));

    loop {
        clearscreen::clear().unwrap();

        if let Some(page) = navigator.get_current_page() {
            if let Err(error) = page.draw_page() {
                println!("Error rendering page: {error}\nPress any key to continue...");
                pause();
            };

            match page.handle_input(read_line().trim()) {
                Err(error) => {
                    println!("Error getting user input: {error}\nPress any key to continue...");
                    pause();
                },
                Ok(action) => {
                    if let Some(action) = action {
                        if let Err(error) = navigator.handle_action(action) {
                            println!(
                                "Error handling processing user input: {error}\nPress any key \
                                 to continue..."
                            );
                            pause();
                        }
                    }
                },
            }
        } else {
            break;
        }
    }
}
