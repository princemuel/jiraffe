use std::fs;
use std::path::PathBuf;

use anyhow::{Ok, Result, anyhow};

use crate::models::{DBState, Epic, Status, Story};

pub struct JiraDatabase {
    database: Box<dyn Database>,
}

impl JiraDatabase {
    pub fn new(file_path: String) -> Self {
        Self { database: Box::new(JSONFileDatabase { file_path: file_path.into() }) }
    }

    pub fn read(&self) -> Result<DBState> { self.database.read() }

    pub fn create_epic(&self, epic: Epic) -> Result<u32> {
        let mut db_state = self.database.read()?;

        db_state.last_item_id += 1;
        let epic_id = db_state.last_item_id;

        db_state.epics.insert(epic_id, epic);

        self.database.write(&db_state)?;

        Ok(epic_id)
    }

    pub fn create_story(&self, story: Story, epic_id: u32) -> Result<u32> {
        let mut db_state = self.database.read()?;
        if let Some(epic) = db_state.epics.get_mut(&epic_id) {
            db_state.last_item_id += 1;

            let story_id = db_state.last_item_id;
            db_state.stories.insert(story_id, story);
            epic.stories.push(story_id);

            self.database.write(&db_state)?;
            Ok(story_id)
        } else {
            Err(anyhow!("Epic with id {epic_id} not found"))
        }
    }

    pub fn delete_epic(&self, epic_id: u32) -> Result<()> {
        let mut db_state = self.database.read()?;

        if let Some(epic) = db_state.epics.get_mut(&epic_id) {
            Ok(())
        } else {
            Err(anyhow!("Epic with id {epic_id} not found"))
        }
    }

    pub fn delete_story(&self, epic_id: u32, story_id: u32) -> Result<()> {
        let mut db_state = self.database.read()?;

        if let Some(epic) = db_state.epics.get_mut(&epic_id) {
            if let Some(story) = db_state.stories.get_mut(&story_id) {
                Ok(())
            } else {
                Err(anyhow!("Story with id {story_id} not found"))
            }
        } else {
            Err(anyhow!("Epic with id {epic_id} not found"))
        }
    }

    pub fn update_epic_status(&self, epic_id: u32, status: Status) -> Result<()> {
        let mut db_state = self.database.read()?;

        if let Some(epic) = db_state.epics.get_mut(&epic_id) {
            Ok(())
        } else {
            Err(anyhow!("Epic with id {epic_id} not found"))
        }
    }

    pub fn update_story_status(&self, story_id: u32, status: Status) -> Result<()> {
        let mut db_state = self.database.read()?;

        if let Some(story) = db_state.stories.get_mut(&story_id) {
            Ok(())
        } else {
            Err(anyhow!("Story with id {story_id} not found"))
        }
    }
}

trait Database {
    fn read(&self) -> Result<DBState>;
    fn write(&self, db_state: &DBState) -> Result<()>;
}

struct JSONFileDatabase {
    pub file_path: PathBuf,
}

impl Database for JSONFileDatabase {
    fn read(&self) -> Result<DBState> {
        let content = fs::read_to_string(&self.file_path)?;
        Ok(serde_json::from_str(&content)?)
    }

    fn write(&self, data: &DBState) -> Result<()> {
        fs::write(&self.file_path, &serde_json::to_vec(data)?)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::test_utils::MockDB;
    use super::*;

    #[test]
    fn create_epic_should_pass() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_string(), "".to_string());

        let result = db.create_epic(epic.clone());
        assert!(result.is_ok());

        let id = result.unwrap();
        let db_state = db.read().unwrap();
        let expected_id = 1;

        assert_eq!(id, expected_id);
        assert_eq!(db_state.last_item_id, expected_id);
        assert_eq!(db_state.epics.get(&id), Some(&epic));
    }

    #[test]
    fn create_story_should_fail_if_invalid_epic_id() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let story = Story::new("".to_string(), "".to_string());
        let non_existent_epic_id = 999;

        let result = db.create_story(story, non_existent_epic_id);
        assert!(result.is_err());
    }

    #[test]
    fn create_story_should_pass() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_string(), "".to_string());
        let story = Story::new("".to_string(), "".to_string());

        let result = db.create_epic(epic);
        assert!(result.is_ok());

        let epic_id = result.unwrap();
        let result = db.create_story(story.clone(), epic_id);
        assert!(result.is_ok());

        let id = result.unwrap();
        let db_state = db.read().unwrap();
        let expected_id = 2;

        assert_eq!(id, expected_id);
        assert_eq!(db_state.last_item_id, expected_id);
        assert!(db_state.epics.get(&epic_id).unwrap().stories.contains(&id));
        assert_eq!(db_state.stories.get(&id), Some(&story));
    }

    #[test]
    fn delete_epic_should_fail_if_invalid_epic_id() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let non_existent_epic_id = 999;

        let result = db.delete_epic(non_existent_epic_id);
        assert!(result.is_err());
    }

    #[test]
    fn delete_epic_should_pass() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_string(), "".to_string());
        let story = Story::new("".to_string(), "".to_string());

        let result = db.create_epic(epic);
        assert!(result.is_ok());

        let epic_id = result.unwrap();
        let result = db.create_story(story, epic_id);
        assert!(result.is_ok());

        let story_id = result.unwrap();
        let result = db.delete_epic(epic_id);
        assert!(result.is_ok());

        let db_state = db.read().unwrap();
        let expected_last_id = 2;
        assert_eq!(db_state.last_item_id, expected_last_id);
        assert_eq!(db_state.epics.get(&epic_id), None);
        assert_eq!(db_state.stories.get(&story_id), None);
    }

    #[test]
    fn delete_story_should_fail_if_invalid_epic_id() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_string(), "".to_string());
        let story = Story::new("".to_string(), "".to_string());

        let result = db.create_epic(epic);
        assert!(result.is_ok());

        let epic_id = result.unwrap();
        let result = db.create_story(story, epic_id);
        assert!(result.is_ok());

        let story_id = result.unwrap();
        let non_existent_epic_id = 999;
        let result = db.delete_story(non_existent_epic_id, story_id);
        assert!(result.is_err());
    }

    #[test]
    fn delete_story_should_fail_if_story_not_found_in_epic() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_string(), "".to_string());
        let story = Story::new("".to_string(), "".to_string());

        let result = db.create_epic(epic);
        assert!(result.is_ok());

        let epic_id = result.unwrap();
        let result = db.create_story(story, epic_id);
        assert!(result.is_ok());

        let non_existent_story_id = 999;
        let result = db.delete_story(epic_id, non_existent_story_id);
        assert!(result.is_err());
    }

    #[test]
    fn delete_story_should_pass() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_string(), "".to_string());
        let story = Story::new("".to_string(), "".to_string());

        let result = db.create_epic(epic);
        assert!(result.is_ok());

        let epic_id = result.unwrap();
        let result = db.create_story(story, epic_id);
        assert!(result.is_ok());

        let story_id = result.unwrap();
        let result = db.delete_story(epic_id, story_id);
        assert!(result.is_ok());

        let db_state = db.read().unwrap();
        let expected_last_id = 2;
        assert_eq!(db_state.last_item_id, expected_last_id);
        assert!(!db_state.epics.get(&epic_id).unwrap().stories.contains(&story_id));
        assert_eq!(db_state.stories.get(&story_id), None);
    }

    #[test]
    fn update_epic_status_should_fail_if_invalid_epic_id() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let non_existent_epic_id = 999;

        let result = db.update_epic_status(non_existent_epic_id, Status::Closed);
        assert!(result.is_err());
    }

    #[test]
    fn update_epic_status_should_pass() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_string(), "".to_string());

        let result = db.create_epic(epic);
        assert!(result.is_ok());

        let epic_id = result.unwrap();

        let result = db.update_epic_status(epic_id, Status::Closed);

        assert!(result.is_ok());

        let db_state = db.read().unwrap();

        assert_eq!(db_state.epics.get(&epic_id).unwrap().status, Status::Closed);
    }

    #[test]
    fn update_story_status_should_fail_if_invalid_story_id() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };

        let non_existent_story_id = 999;

        let result = db.update_story_status(non_existent_story_id, Status::Closed);
        assert!(result.is_err());
    }

    #[test]
    fn update_story_status_should_pass() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_string(), "".to_string());
        let story = Story::new("".to_string(), "".to_string());

        let result = db.create_epic(epic);
        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);
        let story_id = result.unwrap();

        let result = db.update_story_status(story_id, Status::Closed);
        assert!(result.is_ok());

        let db_state = db.read().unwrap();
        assert_eq!(db_state.stories.get(&story_id).unwrap().status, Status::Closed);
    }

    mod database {
        use std::collections::HashMap;
        use std::io::Write;

        use super::*;

        #[test]
        fn read_from_db_should_fail_with_invalid_path() {
            let db = JSONFileDatabase { file_path: "INVALID_PATH".into() };
            assert!(db.read().is_err());
        }

        #[test]
        fn read_from_db_should_fail_with_invalid_json() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0 epics: {} stories {} }"#;
            write!(tmpfile, "{file_contents}").unwrap();

            let db = JSONFileDatabase { file_path: tmpfile.path().to_path_buf() };

            let result = db.read();

            assert!(result.is_err());
        }

        #[test]
        fn read_from_db_should_parse_json_file() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
            write!(tmpfile, "{file_contents}").unwrap();

            let db = JSONFileDatabase { file_path: tmpfile.path().to_path_buf() };

            let result = db.read();
            assert!(result.is_ok());
        }

        #[test]
        fn write_to_db_should_pass() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
            write!(tmpfile, "{file_contents}").unwrap();

            let db = JSONFileDatabase { file_path: tmpfile.path().to_path_buf() };
            let story = Story {
                name:        "epic 1".to_string(),
                description: "epic 1".to_string(),
                status:      Status::Open,
            };
            let epic = Epic {
                name:        "epic 1".to_string(),
                description: "epic 1".to_string(),
                status:      Status::Open,
                stories:     vec![2],
            };

            let mut stories = HashMap::with_capacity(1);
            stories.insert(2, story);

            let mut epics = HashMap::with_capacity(1);
            epics.insert(1, epic);

            let state = DBState { last_item_id: 2, epics, stories };

            let write_result = db.write(&state);
            let read_result = db.read().unwrap();

            assert!(write_result.is_ok());
            assert_eq!(read_result, state);
        }
    }
}

#[cfg(test)]
pub mod test_utils {
    use std::cell::RefCell;
    use std::collections::HashMap;

    use super::*;

    pub struct MockDB {
        last_written_state: RefCell<DBState>,
    }

    impl MockDB {
        pub fn new() -> Self {
            Self {
                last_written_state: RefCell::new(DBState {
                    last_item_id: 0,
                    epics:        HashMap::with_capacity(2),
                    stories:      HashMap::with_capacity(2),
                }),
            }
        }
    }

    impl Database for MockDB {
        fn read(&self) -> Result<DBState> {
            let state = self.last_written_state.borrow().clone();
            Ok(state)
        }

        fn write(&self, data: &DBState) -> Result<()> {
            let latest_state = &self.last_written_state;
            *latest_state.borrow_mut() = data.clone();
            Ok(())
        }
    }
}
