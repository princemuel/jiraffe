use crate::io::read_line;
use crate::models::{Epic, Status, Story};

pub struct Prompts {
    pub create_epic:   Box<dyn Fn() -> Epic>,
    pub create_story:  Box<dyn Fn() -> Story>,
    pub delete_epic:   Box<dyn Fn() -> bool>,
    pub delete_story:  Box<dyn Fn() -> bool>,
    pub update_status: Box<dyn Fn() -> Option<Status>>,
}

impl Prompts {
    pub fn new() -> Self {
        Self {
            create_epic:   Box::new(create_epic_prompt),
            create_story:  Box::new(create_story_prompt),
            delete_epic:   Box::new(delete_epic_prompt),
            delete_story:  Box::new(delete_story_prompt),
            update_status: Box::new(update_status_prompt),
        }
    }
}

impl Default for Prompts {
    fn default() -> Self { Self::new() }
}

fn create_epic_prompt() -> Epic {
    println!("----------------------------");

    println!("Epic Name: ");
    let epic_name = read_line();

    println!("Epic Description: ");
    let epic_desc = read_line();

    Epic::new(epic_name.trim().to_string(), epic_desc.trim().to_string())
}
fn create_story_prompt() -> Story {
    println!("Story Name: ");
    let story_name = read_line();

    println!("Story Description: ");
    let story_desc = read_line();

    Story::new(story_name.trim().to_string(), story_desc.trim().to_string())
}

fn delete_epic_prompt() -> bool {
    println!("----------------------------");
    println!(
        "Are you sure you want to delete this epic? All stories in this epic will also be \
         deleted [Y/n]: "
    );

    read_line().trim().to_lowercase().eq("y")
}

fn delete_story_prompt() -> bool {
    println!("----------------------------");
    println!("Are you sure you want to delete this story? [Y/n]: ");

    read_line().trim().to_lowercase().eq("y")
}

fn update_status_prompt() -> Option<Status> {
    println!("----------------------------");
    println!("New Status (1 - OPEN, 2 - IN-PROGRESS, 3 - RESOLVED, 4 - CLOSED): ");

    read_line().parse().ok()
}
