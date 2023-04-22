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
pub async fn add_user(db: &Surreal<Client>, username: &String, password: &Vec<u8>) {
    if user_id_from_name(db, username).await != None {
        println!("user with that username already exists in the database");
        return;
    }

    let time = Utc::now();
    let id = Id::rand();

    let new_user = User {
        name: username.to_string(),
        hashed_password: password.to_vec(),
        joined_at: Datetime::from(time),
        uuid: id,
    };

    let _created: User = db.create("users").content(new_user).await.unwrap();
}

//adds task to database
pub async fn add_task(db: &Surreal<Client>, name: &String, description: &String, start: &Datetime, end: &Datetime, category: &Id, user: &Id) {
    let timespan = Timespan::new(start, end);
    let id = Id::rand();

    let new_task = Task {
        name: name.to_string(),
        description: description.to_string(),
        timespan,
        category: category.clone(),
        completed: false,
        user: user.clone(),
        uuid: id,
    };

    let _created: Task = db.create("tasks").content(new_task).await.unwrap();
}

//adds event to database
pub async fn add_event(db: &Surreal<Client>, name: &String, description: &String, start: &Datetime, end: &Datetime, category: &Id, user: &Id) {
    let timespan = Timespan::new(start, end);
    let id = Id::rand();

    let new_event = Event {
        name: name.to_string(),
        description: description.to_string(),
        timespan,
        category: category.clone(),
        user: user.clone(),
        uuid: id,
    };

    let _created: Event = db.create("events").content(new_event).await.unwrap();
}

//retrieve events for a given user
pub async fn get_tasks(db: &Surreal<Client>, userid: &Id) -> Vec<Task> {
    let tasks: Vec<Task> = db.select("tasks").await.unwrap();
    let tasks_filtered: Vec<Task> = tasks.into_iter().filter(|x| x.user == userid.clone()).collect();
    tasks_filtered
}

//retrieve events for a given user
pub async fn get_events(db: &Surreal<Client>, userid: &Id) -> Vec<Event> {
    let events: Vec<Event> = db.select("events").await.unwrap();
    let events_filtered: Vec<Event> = events.into_iter().filter(|x| x.user == userid.clone()).collect();
    events_filtered
}

//retrieve user id from username
pub async fn user_id_from_name(db: &Surreal<Client>, name: &String) -> Option<Id> {
    let users: Vec<User> = db.select("users").await.unwrap();
    let users_filtered: Vec<User> = users.into_iter().filter(|x| x.name == *name).collect();
    match users_filtered.get(0) {
        Some(x) => Some(x.uuid.clone()),
        None => None,
    }
}

