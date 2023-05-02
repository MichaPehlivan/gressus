use serde::{Serialize, Deserialize};
use surrealdb::sql::{Uuid};
use chrono::{DateTime, Utc};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub name: String,
    pub hashed_password: Vec<u8>,
    pub joined_at: DateTime::<Utc>,
    pub categories: Vec<Uuid>,
    pub uuid: Uuid,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Task {
    pub name: String,
    pub description: String,
    pub timespan: Timespan,
    pub category: Uuid,
    pub completed: bool,
    pub user: Uuid,
    pub uuid: Uuid,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Event {
    pub name: String,
    pub description: String,
    pub timespan: Timespan,
    pub category: Uuid,
    pub user: Uuid,
    pub uuid: Uuid,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Category {
    pub name: String,
    pub color: u32,
    pub user: Uuid,
    pub uuid: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Timespan {
    pub start: DateTime::<Utc>,
    pub end: DateTime::<Utc>,
}

impl Timespan {
    pub fn new(start: &DateTime::<Utc>, end: &DateTime::<Utc>) -> Self {
        Timespan {
            start: start.clone(),
            end: end.clone()
        }
    }
}
