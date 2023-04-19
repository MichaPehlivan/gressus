use serde::{Serialize, Deserialize};
use surrealdb::sql::{Datetime, Id};

#[derive(Serialize, Deserialize)]
pub struct User<'a> {
    name: String,
    hashed_password: &'a [u8],
    joined_at: Datetime,
    uuid: Id,
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    name: String,
    description: String,
    timespan: Timespan,
    category: Id,
    completed: bool,
    uuid: Id,
}

#[derive(Serialize, Deserialize)]
pub struct Event {
    name: String,
    timespan: Timespan,
    category: Id,
    uuid: Id,
}

#[derive(Serialize, Deserialize)]
pub struct Timespan {
    start: Datetime,
    end: Datetime,
}
