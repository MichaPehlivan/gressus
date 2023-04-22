use chrono::Utc;
use serde::Deserialize;
use surrealdb::{sql::{Datetime, Id, Thing}, Surreal, engine::remote::ws::Client};

use crate::common::model::{User, Timespan, Task, Event};

#[derive(Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

//adds user to database
pub async fn add_user(db: &Surreal<Client>, username: String, password: Vec<u8>) {
    let time = Utc::now();
    let id = Id::rand();

    let new_user = User {
        name: username,
        hashed_password: password,
        joined_at: Datetime::from(time),
        uuid: id,
    };

    let _created: User = db.create("users").content(new_user).await.unwrap();
}

//adds task to database
pub async fn add_task(db: &Surreal<Client>, name: String, description: String, start: Datetime, end: Datetime, category: Id, user: Id) {
    let timespan = Timespan::new(start, end);
    let id = Id::rand();

    let new_task = Task {
        name,
        description,
        timespan,
        category,
        completed: false,
        user,
        uuid: id,
    };

    let _created: Task = db.create("tasks").content(new_task).await.unwrap();
}

//adds event to database
pub async fn add_event(db: &Surreal<Client>, name: String, start: Datetime, end: Datetime, category: Id, user: Id) {
    let timespan = Timespan::new(start, end);
    let id = Id::rand();

    let new_event = Event {
        name,
        timespan,
        category,
        user,
        uuid: id,
    };

    let _created: Event = db.create("events").content(new_event).await.unwrap();
}

pub async fn user_id_from_name(db: &Surreal<Client>, name: String) -> Option<Id> {
    let result: Vec<User> = db.select("users").await.unwrap();
    let users: Vec<User> = result.into_iter().filter(|x| x.name == name).collect();
    match users.get(0) {
        Some(x) => Some(x.uuid.clone()),
        None => None,
    }
}

