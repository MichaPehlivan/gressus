use chrono::Utc;
use surrealdb::{sql::{Datetime, Uuid}, Surreal, engine::remote::ws::Client};

use crate::common::model::{User, Timespan, Task, Event};

///adds user to database
pub async fn add_user(db: &Surreal<Client>, username: &str, password: &Vec<u8>) {
    if user_id_from_name(db, username).await != None {
        println!("user with that username already exists in the database");
        return;
    }

    let time = Utc::now();
    let id = Uuid::new();

    let new_user = User {
        name: username.to_string(),
        hashed_password: password.to_vec(),
        joined_at: Datetime::from(time),
        categories: Vec::new(),
        uuid: id.clone(),
    };

    let _created: User = db.create(("users", id.to_raw())).content(new_user).await.unwrap();
}

///adds task to database
pub async fn add_task(db: &Surreal<Client>, name: &str, description: &str, start: &Datetime, end: &Datetime, category: &Uuid, user: &Uuid) {
    let timespan = Timespan::new(start, end);
    let id = Uuid::new();

    let new_task = Task {
        name: name.to_string(),
        description: description.to_string(),
        timespan,
        category: category.clone(),
        completed: false,
        user: user.clone(),
        uuid: id.clone(),
    };

    let _created: Task = db.create(("tasks", id.to_raw())).content(new_task).await.unwrap();
}

///adds event to database
pub async fn add_event(db: &Surreal<Client>, name: &str, description: &str, start: &Datetime, end: &Datetime, category: &Uuid, user: &Uuid) {
    let timespan = Timespan::new(start, end);
    let id = Uuid::new();

    let new_event = Event {
        name: name.to_string(),
        description: description.to_string(),
        timespan,
        category: category.clone(),
        user: user.clone(),
        uuid: id.clone(),
    };

    let _created: Event = db.create(("events", id.to_raw())).content(new_event).await.unwrap();
}

///get user from Uuid
pub async fn get_user(db: &Surreal<Client>, user_id: &Uuid) -> Option<User> {
    let user: Option<User> = db.select(("users", user_id.to_raw())).await.unwrap();
    user
}

///retrieve events for a given user
pub async fn get_tasks(db: &Surreal<Client>, userid: &Uuid) -> Vec<Task> {
    let tasks: Vec<Task> = db.select("tasks").await.unwrap();
    let tasks_filtered: Vec<Task> = tasks.into_iter().filter(|x| x.user == userid.clone()).collect();
    tasks_filtered
}

///retrieve events for a given user
pub async fn get_events(db: &Surreal<Client>, userid: &Uuid) -> Vec<Event> {
    let events: Vec<Event> = db.select("events").await.unwrap();
    let events_filtered: Vec<Event> = events.into_iter().filter(|x| x.user == userid.clone()).collect();
    events_filtered
}

///retrieve categories for a given user
pub async fn get_categories(db: &Surreal<Client>, userid: &Uuid) -> Vec<Uuid> {
    let user = get_user(db, userid).await.unwrap();
    user.categories
}

///retrieve user id from username
pub async fn user_id_from_name(db: &Surreal<Client>, name: &str) -> Option<Uuid> {
    let users: Vec<User> = db.select("users").await.unwrap();
    let users_filtered: Vec<User> = users.into_iter().filter(|x| x.name == *name).collect();
    match users_filtered.get(0) {
        Some(x) => Some(x.uuid.clone()),
        None => None,
    }
}

///change username
pub async fn change_username(db: &Surreal<Client>, user: &Uuid, new_username: &str) {
    let query = format!("UPDATE users:`{}` SET name = \"{}\"", user.to_raw(), new_username);
    db.query(query).await.unwrap();
}

///add category to user
pub async fn add_category(db: &Surreal<Client>, user: &Uuid, new_category: &Uuid) {
    let mut categories = get_categories(db, user).await;
    categories.push(new_category.clone());
    let categories_str = uuid_vec_to_string(categories);
    let query = format!("UPDATE users:`{}` SET categories = {}", user.to_raw(), categories_str);
    db.query(query).await.unwrap();
}

///deletes a user
pub async fn delete_user(db: &Surreal<Client>, user: &Uuid) {
    let query = format!("DELETE users:`{}`", user.to_raw());
    db.query(query).await.unwrap();
}

///deletes a task
pub async fn delete_task(db: &Surreal<Client>, task: &Uuid) {
    let query = format!("DELETE tasks:`{}`", task.to_raw());
    db.query(query).await.unwrap();
}

///deletes an event
pub async fn delete_event(db: &Surreal<Client>, event: &Uuid) {
    let query = format!("DELETE events:`{}`", event.to_raw());
    db.query(query).await.unwrap();
}

/// convert vec of uuid's to string for queries
fn uuid_vec_to_string(vec: Vec<Uuid>) -> String {
    let mut result = String::from("[");
    for x in vec {
        result.push_str(&format!("{},", x.to_string()));
    }
    result.push(']');
    result
}
