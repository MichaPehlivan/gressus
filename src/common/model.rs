use serde::{Serialize, Deserialize};
use surrealdb::sql::{Datetime, Id};

#[derive(Serialize, Deserialize)]
pub struct User<'a> {
    pub name: String,
    pub hashed_password: &'a [u8],
    pub joined_at: Datetime,
    pub uuid: Id,
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub description: String,
    pub timespan: Timespan,
    pub category: Id,
    pub completed: bool,
    pub user: Id,
    pub uuid: Id,
}

#[derive(Serialize, Deserialize)]
pub struct Event {
    pub name: String,
    pub timespan: Timespan,
    pub category: Id,
    pub user: Id,
    pub uuid: Id,
}

#[derive(Serialize, Deserialize)]
pub struct Timespan {
    pub start: Datetime,
    pub end: Datetime,
}

impl Timespan {
    pub fn new(start: Datetime, end: Datetime) -> Self {
        Timespan {
            start,
            end
        }
    }
}
