use std::any::Any;
use std::rc::Rc;

use anyhow::{Context, Result, anyhow};
use itertools::Itertools;

use crate::database::JiraDatabase;
use crate::models::Action;

mod helpers;
use helpers::*;

pub trait Page {
    fn as_any(&self) -> &dyn Any;
    fn draw_page(&self) -> Result<()>;
    fn handle_input(&self, input: &str) -> Result<Option<Action>>;
}

const EPIC_TABLE_HEADER: &str =
    "----------------------------- EPICS ------------------------------";
const STORY_TABLE_HEADER: &str =
    "---------------------------- STORIES -----------------------------";
const EPIC_DETAIL_HEADER: &str =
    "------------------------------ EPIC ------------------------------";
const STORY_DETAIL_HEADER: &str =
    "------------------------------ STORY -----------------------------";

const EPIC_COLUMN_HEADER: &str =
    "     id     |               name               |      status      ";
const STORY_COLUMN_HEADER: &str =
    "     id     |               name               |      status      ";
const DETAIL_COLUMN_HEADER: &str =
    "  id  |     name     |         description         |    status    ";

fn print_table_row(
    id: u32,
    name: &str,
    status: &str,
    id_width: usize,
    name_width: usize,
    status_width: usize,
) {
    let id_col = get_column_string(&id.to_string(), id_width);
    let name_col = get_column_string(name, name_width);
    let status_col = get_column_string(status, status_width);
    println!("{id_col} | {name_col} | {status_col}");
}

fn print_detail_row(id: u32, name: &str, description: &str, status: &str) {
    let id_col = get_column_string(&id.to_string(), 5);
    let name_col = get_column_string(name, 12);
    let desc_col = get_column_string(description, 27);
    let status_col = get_column_string(status, 13);
    println!("{id_col} | {name_col} | {desc_col} | {status_col}");
}

pub struct HomePage {
    pub database: Rc<JiraDatabase>,
}
impl Page for HomePage {
    fn draw_page(&self) -> Result<()> {
        println!("{EPIC_TABLE_HEADER}");
        println!("{EPIC_COLUMN_HEADER}");

        let db_state = self.database.read().context("Failed to read from database")?;

        db_state.epics.iter().sorted_by_key(|(id, _)| *id).for_each(|(id, epic)| {
            print_table_row(*id, &epic.name, &epic.status.to_string(), 11, 32, 17);
        });

        println!("\n\n[q] quit | [c] create epic | [:id:] navigate to epic");
        Ok(())
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        match input {
            "q" => Ok(Some(Action::Exit)),
            "c" => Ok(Some(Action::CreateEpic)),
            input => match input.parse::<u32>() {
                Ok(epic_id) => {
                    let db_state =
                        self.database.read().context("Failed to read from database")?;
                    if db_state.epics.contains_key(&epic_id) {
                        Ok(Some(Action::NavigateToEpicDetail { epic_id }))
                    } else {
                        Ok(None)
                    }
                },
                Err(_) => Ok(None),
            },
        }
    }

    fn as_any(&self) -> &dyn Any { self }
}

pub struct EpicDetail {
    pub epic_id:  u32,
    pub database: Rc<JiraDatabase>,
}

impl Page for EpicDetail {
    fn draw_page(&self) -> Result<()> {
        let db_state = self.database.read().context("Failed to read from database")?;
        let epic = db_state
            .epics
            .get(&self.epic_id)
            .ok_or_else(|| anyhow!("Epic with id {} not found!", &self.epic_id))?;

        println!("{EPIC_DETAIL_HEADER}");
        println!("{DETAIL_COLUMN_HEADER}");
        print_detail_row(self.epic_id, &epic.name, &epic.description, &epic.status.to_string());

        println!();

        println!("{STORY_TABLE_HEADER}");
        println!("{STORY_COLUMN_HEADER}");

        epic.stories
            .iter()
            .sorted()
            .filter_map(|id| db_state.stories.get(id).map(|story| (*id, story)))
            .for_each(|(id, story)| {
                print_table_row(id, &story.name, &story.status.to_string(), 11, 32, 17);
            });

        println!(
            "\n\n[p] previous | [u] update epic | [d] delete epic | [c] create story | [:id:] \
             navigate to story"
        );
        Ok(())
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        match input {
            "p" => Ok(Some(Action::NavigateToPreviousPage)),
            "u" => Ok(Some(Action::UpdateEpicStatus { epic_id: self.epic_id })),
            "d" => Ok(Some(Action::DeleteEpic { epic_id: self.epic_id })),
            "c" => Ok(Some(Action::CreateStory { epic_id: self.epic_id })),
            input => match input.parse::<u32>() {
                Ok(story_id) => {
                    let db_state =
                        self.database.read().context("Failed to read from database")?;
                    if db_state.stories.contains_key(&story_id) {
                        Ok(Some(Action::NavigateToStoryDetail {
                            epic_id: self.epic_id,
                            story_id,
                        }))
                    } else {
                        Ok(None)
                    }
                },
                Err(_) => Ok(None),
            },
        }
    }

    fn as_any(&self) -> &dyn Any { self }
}

pub struct StoryDetail {
    pub epic_id:  u32,
    pub story_id: u32,
    pub database: Rc<JiraDatabase>,
}

impl Page for StoryDetail {
    fn draw_page(&self) -> Result<()> {
        let db_state = self.database.read().context("Failed to read from database")?;
        let story = db_state
            .stories
            .get(&self.story_id)
            .ok_or_else(|| anyhow!("Story with id {} not found!", self.story_id))?;

        println!("{STORY_DETAIL_HEADER}");
        println!("{DETAIL_COLUMN_HEADER}");
        print_detail_row(
            self.story_id,
            &story.name,
            &story.description,
            &story.status.to_string(),
        );

        println!("\n\n[p] previous | [u] update story | [d] delete story");
        Ok(())
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        match input {
            "p" => Ok(Some(Action::NavigateToPreviousPage)),
            "u" => Ok(Some(Action::UpdateStoryStatus { story_id: self.story_id })),
            "d" => Ok(Some(Action::DeleteStory {
                epic_id:  self.epic_id,
                story_id: self.story_id,
            })),
            _ => Ok(None),
        }
    }

    fn as_any(&self) -> &dyn Any { self }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::test_utils::MockDB;
    use crate::models::{Epic, Story};

    mod home_page {
        use super::*;

        #[test]
        fn draw_page_should_not_fail() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let page = HomePage { database: db };
            assert!(page.draw_page().is_ok());
        }

        #[test]
        fn handle_input_should_not_fail() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let page = HomePage { database: db };
            assert!(page.handle_input("").is_ok());
        }

        #[test]
        fn handle_input_should_return_the_correct_actions() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let epic = Epic::new("".to_string(), "".to_string());

            let epic_id = db.create_epic(epic).unwrap();

            let page = HomePage { database: db };

            let q = "q";
            let c = "c";
            let valid_epic_id = epic_id.to_string();
            let invalid_epic_id = "999";
            let junk_input = "j983f2j";
            let junk_input_with_valid_prefix = "q983f2j";
            let input_with_trailing_white_spaces = "q\n";

            assert_eq!(page.handle_input(q).unwrap(), Some(Action::Exit));
            assert_eq!(page.handle_input(c).unwrap(), Some(Action::CreateEpic));
            assert_eq!(
                page.handle_input(&valid_epic_id).unwrap(),
                Some(Action::NavigateToEpicDetail { epic_id: 1 })
            );
            assert_eq!(page.handle_input(invalid_epic_id).unwrap(), None);
            assert_eq!(page.handle_input(junk_input).unwrap(), None);
            assert_eq!(page.handle_input(junk_input_with_valid_prefix).unwrap(), None);
            assert_eq!(page.handle_input(input_with_trailing_white_spaces).unwrap(), None);
        }
    }

    mod epic_detail_page {
        use super::*;

        #[test]
        fn draw_page_should_not_fail() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });
            let epic_id = db.create_epic(Epic::new("".to_string(), "".to_string())).unwrap();

            let page = EpicDetail { epic_id, database: db };
            assert!(page.draw_page().is_ok());
        }

        #[test]
        fn handle_input_should_not_fail() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });
            let epic_id = db.create_epic(Epic::new("".to_string(), "".to_string())).unwrap();

            let page = EpicDetail { epic_id, database: db };
            assert!(page.handle_input("").is_ok());
        }

        #[test]
        fn draw_page_should_fail_for_invalid_epic_id() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let page = EpicDetail { epic_id: 999, database: db };
            assert!(page.draw_page().is_err());
        }

        #[test]
        fn handle_input_should_return_the_correct_actions() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let epic_id = db.create_epic(Epic::new("".to_string(), "".to_string())).unwrap();
            let story_id =
                db.create_story(Story::new("".to_string(), "".to_string()), epic_id).unwrap();

            let page = EpicDetail { epic_id, database: db };

            let p = "p";
            let u = "u";
            let d = "d";
            let c = "c";
            let invalid_story_id = "999";
            let junk_input = "j983f2j";
            let junk_input_with_valid_prefix = "p983f2j";
            let input_with_trailing_white_spaces = "p\n";

            assert_eq!(page.handle_input(p).unwrap(), Some(Action::NavigateToPreviousPage));
            assert_eq!(
                page.handle_input(u).unwrap(),
                Some(Action::UpdateEpicStatus { epic_id: 1 })
            );
            assert_eq!(page.handle_input(d).unwrap(), Some(Action::DeleteEpic { epic_id: 1 }));
            assert_eq!(page.handle_input(c).unwrap(), Some(Action::CreateStory { epic_id: 1 }));
            assert_eq!(
                page.handle_input(&story_id.to_string()).unwrap(),
                Some(Action::NavigateToStoryDetail { epic_id: 1, story_id: 2 })
            );
            assert_eq!(page.handle_input(invalid_story_id).unwrap(), None);
            assert_eq!(page.handle_input(junk_input).unwrap(), None);
            assert_eq!(page.handle_input(junk_input_with_valid_prefix).unwrap(), None);
            assert_eq!(page.handle_input(input_with_trailing_white_spaces).unwrap(), None);
        }
    }

    mod story_detail_page {
        use super::*;

        #[test]
        fn draw_page_should_not_fail() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let epic_id = db.create_epic(Epic::new("".to_string(), "".to_string())).unwrap();
            let story_id =
                db.create_story(Story::new("".to_string(), "".to_string()), epic_id).unwrap();

            let page = StoryDetail { epic_id, story_id, database: db };
            assert!(page.draw_page().is_ok());
        }

        #[test]
        fn handle_input_should_not_fail() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let epic_id = db.create_epic(Epic::new("".to_string(), "".to_string())).unwrap();
            let story_id =
                db.create_story(Story::new("".to_string(), "".to_string()), epic_id).unwrap();

            let page = StoryDetail { epic_id, story_id, database: db };
            assert!(page.handle_input("").is_ok());
        }

        #[test]
        fn draw_page_should_fail_for_invalid_story_id() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let epic_id = db.create_epic(Epic::new("".to_string(), "".to_string())).unwrap();
            let _ =
                db.create_story(Story::new("".to_string(), "".to_string()), epic_id).unwrap();

            let page = StoryDetail { epic_id, story_id: 999, database: db };
            assert!(page.draw_page().is_err());
        }

        #[test]
        fn handle_input_should_return_the_correct_actions() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let epic_id = db.create_epic(Epic::new("".to_string(), "".to_string())).unwrap();
            let story_id =
                db.create_story(Story::new("".to_string(), "".to_string()), epic_id).unwrap();

            let page = StoryDetail { epic_id, story_id, database: db };

            let p = "p";
            let u = "u";
            let d = "d";
            let some_number = "1";
            let junk_input = "j983f2j";
            let junk_input_with_valid_prefix = "p983f2j";
            let input_with_trailing_white_spaces = "p\n";

            assert_eq!(page.handle_input(p).unwrap(), Some(Action::NavigateToPreviousPage));
            assert_eq!(
                page.handle_input(u).unwrap(),
                Some(Action::UpdateStoryStatus { story_id })
            );
            assert_eq!(
                page.handle_input(d).unwrap(),
                Some(Action::DeleteStory { epic_id, story_id })
            );
            assert_eq!(page.handle_input(some_number).unwrap(), None);
            assert_eq!(page.handle_input(junk_input).unwrap(), None);
            assert_eq!(page.handle_input(junk_input_with_valid_prefix).unwrap(), None);
            assert_eq!(page.handle_input(input_with_trailing_white_spaces).unwrap(), None);
        }
    }
}
