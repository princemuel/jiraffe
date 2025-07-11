use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    NavigateToEpicDetail { epic_id: u32 },
    NavigateToStoryDetail { epic_id: u32, story_id: u32 },
    NavigateToPreviousPage,
    CreateEpic,
    UpdateEpicStatus { epic_id: u32 },
    DeleteEpic { epic_id: u32 },
    CreateStory { epic_id: u32 },
    UpdateStoryStatus { story_id: u32 },
    DeleteStory { epic_id: u32, story_id: u32 },
    Exit,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    Closed,
    Open,
    InProgress,
    Resolved,
}

impl From<&Status> for &'static str {
    fn from(status: &Status) -> Self {
        match status {
            Status::Open => "OPEN",
            Status::InProgress => "IN PROGRESS",
            Status::Resolved => "RESOLVED",
            Status::Closed => "CLOSED",
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(self.into()) }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Epic {
    pub name:        String,
    pub description: String,
    pub status:      Status,
    pub stories:     Vec<u32>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Self { name, description, status: Status::Open, stories: Vec::with_capacity(8) }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Story {
    pub name:        String,
    pub description: String,
    pub status:      Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Self { name, description, status: Status::Open }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DBState {
    pub last_item_id: u32,
    pub epics:        HashMap<u32, Epic>,
    pub stories:      HashMap<u32, Story>,
}
impl DBState {
    pub fn new() -> Self {
        Self { last_item_id: 0, epics: HashMap::new(), stories: HashMap::new() }
    }
}

impl Default for DBState {
    fn default() -> Self { Self::new() }
}
