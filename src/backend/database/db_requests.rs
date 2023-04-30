use chrono::Utc;
use surrealdb::{sql::{Datetime, Uuid}, Surreal, engine::remote::ws::Client};

use crate::common::model::{User, Timespan, Task, Event};

///adds user to database
pub async fn add_user(db: &Surreal<Client>, username: &str, password: &Vec<u8>) -> User {
    if user_id_from_name(db, username).await != None {
        println!("user with that username already exists in the database");
        return User::default();
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

    let created: User = db.create(("users", id.to_raw())).content(new_user).await.unwrap();
    created
}

///adds task to database
pub async fn add_task(db: &Surreal<Client>, name: &str, description: &str, start: &Datetime, end: &Datetime, category: &Uuid, user: &Uuid) -> Task {
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

    let created: Task = db.create(("tasks", id.to_raw())).content(new_task).await.unwrap();
    created
}

///adds event to database
pub async fn add_event(db: &Surreal<Client>, name: &str, description: &str, start: &Datetime, end: &Datetime, category: &Uuid, user: &Uuid) -> Event {
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

    let created: Event = db.create(("events", id.to_raw())).content(new_event).await.unwrap();
    created
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
pub async fn change_username(db: &Surreal<Client>, user: &Uuid, new_username: &str) -> Option<User> {
    let mut new_user = get_user(db, user).await.unwrap();
    new_user.name = new_username.to_string();
    let updated: Option<User> = db.update(("users", user.to_raw())).content(new_user).await.unwrap();
    updated
}

///change password
pub async fn change_password(db: &Surreal<Client>, user: &Uuid, new_password: &Vec<u8>) -> Option<User> {
    let mut new_user = get_user(db, user).await.unwrap();
    new_user.hashed_password = new_password.to_vec();
    let updated: Option<User> = db.update(("users", user.to_raw())).content(new_user).await.unwrap();
    updated
}

///add category to user
pub async fn add_category(db: &Surreal<Client>, user: &Uuid, new_category: &Uuid) -> Option<User> {
    let mut categories = get_categories(db, user).await;
    categories.push(new_category.clone());
    let mut new_user = get_user(db, user).await.unwrap();
    new_user.categories = categories;
    let updated: Option<User> = db.update(("users", user.to_raw())).content(new_user).await.unwrap();
    updated
}

///change task name
pub async fn task_edit_name(db: &Surreal<Client>, task: &Uuid, new_name: &str) -> Option<Task> {

}

///change task description
pub async fn task_edit_desc(db: &Surreal<Client>, task: &Uuid, new_desc: &str) -> Option<Task> {

}

///change task timespan
pub async fn task_edit_timespan(db: &Surreal<Client>, task: &Uuid, new_timespan: &Timespan) -> Option<Task> {

}

///change task category
pub async fn task_change_category(db: &Surreal<Client>, task: &Uuid, new_category: &Uuid) -> Option<Task> {

}

///set the 'completed' field of a task
pub async fn task_set_completion(db: &Surreal<Client>, task: &Uuid, completion: bool) -> Option<Task> {
    
}

///change event name
pub async fn event_edit_name(db: &Surreal<Client>, event: &Uuid, new_name: &str) -> Option<Event> {

}

///change event description
pub async fn event_edit_desc(db: &Surreal<Client>, event: &Uuid, new_desc: &str) -> Option<Event> {

}

///change event timespan
pub async fn event_edit_timespan(db: &Surreal<Client>, event: &Uuid, new_timespan: &Timespan) -> Option<Event> {

}

///change event category
pub async fn event_change_category(db: &Surreal<Client>, event: &Uuid, new_category: &Uuid) -> Option<Event> {

}

///deletes a user
pub async fn delete_user(db: &Surreal<Client>, user: &Uuid) -> Option<User> {
    let deleted: Option<User> = db.delete(("users", user.to_raw())).await.unwrap();
    deleted
}

///deletes a task
pub async fn delete_task(db: &Surreal<Client>, task: &Uuid) -> Option<Task> {
    let deleted: Option<Task> = db.delete(("tasks", task.to_raw())).await.unwrap();
    deleted
}

///deletes an event
pub async fn delete_event(db: &Surreal<Client>, event: &Uuid) -> Option<Event> {
    let deleted: Option<Event> = db.delete(("events", event.to_raw())).await.unwrap();
    deleted
}
