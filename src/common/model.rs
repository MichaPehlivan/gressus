use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Uuid};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct User {
	pub name: String,
	pub hashed_password: Vec<u8>,
	pub joined_at: Datetime,
	pub categories: Vec<Uuid>,
	pub uuid: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
	pub name: String,
	pub description: String,
	pub timespan: Timespan,
	pub category: Uuid,
	pub completed: bool,
	pub user: Uuid,
	pub uuid: Uuid,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
	pub name: String,
	pub description: String,
	pub timespan: Timespan,
	pub category: Uuid,
	pub user: Uuid,
	pub uuid: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
	pub name: String,
	pub color: u32,
	pub user: Uuid,
	pub uuid: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Timespan {
	pub start: Datetime,
	pub end: Datetime,
}

impl Timespan {
	pub fn new(start: &Datetime, end: &Datetime) -> Self {
		Timespan {
			start: start.clone(),
			end: end.clone(),
		}
	}
}
