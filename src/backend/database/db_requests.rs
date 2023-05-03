use chrono::Utc;
use surrealdb::{
	engine::remote::ws::Client,
	sql::{Datetime, Uuid},
	Surreal,
};

use crate::backend::database::db_error::DBerror;
use crate::common::model::{Category, Event, Task, Timespan, User};

///adds user to database
pub async fn add_user(
	db: &Surreal<Client>,
	username: &str,
	password: &Vec<u8>,
) -> Result<User, DBerror> {
	match user_id_from_name(db, username).await {
		Ok(_) => return Err(DBerror::UserAlreadyExists(username.to_string())),
		Err(_) => (),
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
	let created: User = db.create(("users", id.to_raw())).content(new_user).await?;

	Ok(created)
}

///adds task to database
pub async fn add_task(
	db: &Surreal<Client>,
	name: &str,
	description: &str,
	start: &Datetime,
	end: &Datetime,
	category: &Uuid,
	user: &Uuid,
) -> Result<Task, DBerror> {
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

	let created: Task = db.create(("tasks", id.to_raw())).content(new_task).await?;

	Ok(created)
}

///adds event to database
pub async fn add_event(
	db: &Surreal<Client>,
	name: &str,
	description: &str,
	start: &Datetime,
	end: &Datetime,
	category: &Uuid,
	user: &Uuid,
) -> Result<Event, DBerror> {
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
	let created: Event = db
		.create(("events", id.to_raw()))
		.content(new_event)
		.await?;

	Ok(created)
}

///adds category to database
pub async fn add_category(
	db: &Surreal<Client>,
	name: &str,
	color: u32,
	user: &Uuid,
) -> Result<Category, DBerror> {
	let id = Uuid::new();

	let new_category = Category {
		name: name.to_string(),
		color,
		user: user.clone(),
		uuid: id.clone(),
	};

	let created: Category = db
		.create(("categories", id.to_raw()))
		.content(new_category)
		.await?;

	let mut categories = get_categories(db, user).await?;
	categories.push(id.clone());
	let mut new_user = get_user(db, user).await?;
	new_user.categories = categories;
	let _updated: Option<User> = db
		.update(("users", user.to_raw()))
		.content(new_user)
		.await?;

	Ok(created)
}

///get user from Uuid
pub async fn get_user(db: &Surreal<Client>, user_id: &Uuid) -> Result<User, DBerror> {
	let user: Option<User> = db.select(("users", user_id.to_raw())).await?;
	user.ok_or(DBerror::UserNotFound(user_id.clone()))
}

///get task from Uuid
pub async fn get_task(db: &Surreal<Client>, task_id: &Uuid) -> Result<Task, DBerror> {
	let task: Option<Task> = db.select(("tasks", task_id.to_raw())).await?;
	task.ok_or(DBerror::TaskNotFound(task_id.clone()))
}

///get event from Uuid
pub async fn get_event(db: &Surreal<Client>, event_id: &Uuid) -> Result<Event, DBerror> {
	let event: Option<Event> = db.select(("events", event_id.to_raw())).await?;
	event.ok_or(DBerror::EventNotFound(event_id.clone()))
}

///get category from Uuid
pub async fn get_category(db: &Surreal<Client>, category_id: &Uuid) -> Result<Category, DBerror> {
	let category: Option<Category> = db.select(("categories", category_id.to_raw())).await?;
	category.ok_or(DBerror::CategoryNotFound(category_id.clone()))
}

///retrieve tasks for a given user
pub async fn get_tasks(db: &Surreal<Client>, userid: &Uuid) -> Result<Vec<Task>, DBerror> {
	let tasks: Vec<Task> = db.select("tasks").await?;
	let tasks_filtered: Vec<Task> = tasks
		.into_iter()
		.filter(|x| x.user == userid.clone())
		.collect();
	Ok(tasks_filtered)
}

///retrieve events for a given user
pub async fn get_events(db: &Surreal<Client>, userid: &Uuid) -> Result<Vec<Event>, DBerror> {
	let events: Vec<Event> = db.select("events").await?;
	let events_filtered: Vec<Event> = events
		.into_iter()
		.filter(|x| x.user == userid.clone())
		.collect();
	Ok(events_filtered)
}

///retrieve categories for a given user
pub async fn get_categories(db: &Surreal<Client>, userid: &Uuid) -> Result<Vec<Uuid>, DBerror> {
	let user = get_user(db, userid).await?;
	Ok(user.categories)
}

///retrieve user id from username
pub async fn user_id_from_name(db: &Surreal<Client>, name: &str) -> Result<Uuid, DBerror> {
	let users: Vec<User> = db.select("users").await?;
	let users_filtered: Vec<User> = users.into_iter().filter(|x| x.name == *name).collect();
	match users_filtered.get(0) {
		Some(x) => Ok(x.uuid.clone()),
		None => Err(DBerror::UserNameNotFound(name.to_string())),
	}
}

///change username
pub async fn change_username(
	db: &Surreal<Client>,
	user: &Uuid,
	new_username: &str,
) -> Result<User, DBerror> {
	let mut new_user = get_user(db, user).await?;
	new_user.name = new_username.to_string();
	let updated: Option<User> = db
		.update(("users", user.to_raw()))
		.content(new_user)
		.await?;
	updated.ok_or(DBerror::UserNotFound(user.clone()))
}

///change password
pub async fn change_password(
	db: &Surreal<Client>,
	user: &Uuid,
	new_password: &Vec<u8>,
) -> Result<User, DBerror> {
	let mut new_user = get_user(db, user).await?;
	new_user.hashed_password = new_password.to_vec();
	let updated: Option<User> = db
		.update(("users", user.to_raw()))
		.content(new_user)
		.await?;
	updated.ok_or(DBerror::UserNotFound(user.clone()))
}

///change task name
pub async fn task_edit_name(
	db: &Surreal<Client>,
	task: &Uuid,
	new_name: &str,
) -> Result<Task, DBerror> {
	let mut new_task = get_task(db, task).await?;
	new_task.name = new_name.to_string();
	let updated: Option<Task> = db
		.update(("tasks", task.to_raw()))
		.content(new_task)
		.await?;
	updated.ok_or(DBerror::TaskNotFound(task.clone()))
}

///change task description
pub async fn task_edit_desc(
	db: &Surreal<Client>,
	task: &Uuid,
	new_desc: &str,
) -> Result<Task, DBerror> {
	let mut new_task = get_task(db, task).await?;
	new_task.description = new_desc.to_string();
	let updated: Option<Task> = db
		.update(("tasks", task.to_raw()))
		.content(new_task)
		.await?;
	updated.ok_or(DBerror::TaskNotFound(task.clone()))
}

///change task timespan
pub async fn task_edit_timespan(
	db: &Surreal<Client>,
	task: &Uuid,
	new_timespan: &Timespan,
) -> Result<Task, DBerror> {
	let mut new_task = get_task(db, task).await?;
	new_task.timespan = new_timespan.clone();
	let updated: Option<Task> = db
		.update(("tasks", task.to_raw()))
		.content(new_task)
		.await?;
	updated.ok_or(DBerror::TaskNotFound(task.clone()))
}

///change task category
pub async fn task_change_category(
	db: &Surreal<Client>,
	task: &Uuid,
	new_category: &Uuid,
) -> Result<Task, DBerror> {
	let mut new_task = get_task(db, task).await?;
	new_task.category = new_category.clone();
	let updated: Option<Task> = db
		.update(("tasks", task.to_raw()))
		.content(new_task)
		.await?;
	updated.ok_or(DBerror::TaskNotFound(task.clone()))
}

///set the 'completed' field of a task
pub async fn task_set_completion(
	db: &Surreal<Client>,
	task: &Uuid,
	completion: bool,
) -> Result<Task, DBerror> {
	let mut new_task = get_task(db, task).await?;
	new_task.completed = completion;
	let updated: Option<Task> = db
		.update(("tasks", task.to_raw()))
		.content(new_task)
		.await?;
	updated.ok_or(DBerror::TaskNotFound(task.clone()))
}

///change event name
pub async fn event_edit_name(
	db: &Surreal<Client>,
	event: &Uuid,
	new_name: &str,
) -> Result<Event, DBerror> {
	let mut new_event = get_event(db, event).await?;
	new_event.name = new_name.to_string();
	let updated: Option<Event> = db
		.update(("events", event.to_raw()))
		.content(new_event)
		.await?;
	updated.ok_or(DBerror::EventNotFound(event.clone()))
}

///change event description
pub async fn event_edit_desc(
	db: &Surreal<Client>,
	event: &Uuid,
	new_desc: &str,
) -> Result<Event, DBerror> {
	let mut new_event = get_event(db, event).await?;
	new_event.description = new_desc.to_string();
	let updated: Option<Event> = db
		.update(("events", event.to_raw()))
		.content(new_event)
		.await?;
	updated.ok_or(DBerror::EventNotFound(event.clone()))
}

///change event timespan
pub async fn event_edit_timespan(
	db: &Surreal<Client>,
	event: &Uuid,
	new_timespan: &Timespan,
) -> Result<Event, DBerror> {
	let mut new_event = get_event(db, event).await?;
	new_event.timespan = new_timespan.clone();
	let updated: Option<Event> = db
		.update(("events", event.to_raw()))
		.content(new_event)
		.await?;
	updated.ok_or(DBerror::EventNotFound(event.clone()))
}

///change event category
pub async fn event_change_category(
	db: &Surreal<Client>,
	event: &Uuid,
	new_category: &Uuid,
) -> Result<Event, DBerror> {
	let mut new_event = get_event(db, event).await?;
	new_event.category = new_category.clone();
	let updated: Option<Event> = db.update(("events", event.to_raw())).await?;
	updated.ok_or(DBerror::EventNotFound(event.clone()))
}

///change category name
pub async fn category_edit_name(
	db: &Surreal<Client>,
	category: &Uuid,
	new_name: &str,
) -> Result<Category, DBerror> {
	let mut new_category = get_category(db, category).await?;
	new_category.name = new_name.to_string();
	let updated: Option<Category> = db.update(("categories", category.to_raw())).await?;
	updated.ok_or(DBerror::CategoryNotFound(category.clone()))
}

///change category color
pub async fn category_change_color(
	db: &Surreal<Client>,
	category: &Uuid,
	new_color: u32,
) -> Result<Category, DBerror> {
	let mut new_category = get_category(db, category).await?;
	new_category.color = new_color;
	let updated: Option<Category> = db.update(("categories", category.to_raw())).await?;
	updated.ok_or(DBerror::CategoryNotFound(category.clone()))
}

///deletes a user
pub async fn delete_user(db: &Surreal<Client>, user: &Uuid) -> Result<User, DBerror> {
	let deleted: Option<User> = db.delete(("users", user.to_raw())).await?;
	deleted.ok_or(DBerror::UserNotFound(user.clone()))
}

///deletes a task
pub async fn delete_task(db: &Surreal<Client>, task: &Uuid) -> Result<Task, DBerror> {
	let deleted: Option<Task> = db.delete(("tasks", task.to_raw())).await?;
	deleted.ok_or(DBerror::TaskNotFound(task.clone()))
}

///deletes an event
pub async fn delete_event(db: &Surreal<Client>, event: &Uuid) -> Result<Event, DBerror> {
	let deleted: Option<Event> = db.delete(("events", event.to_raw())).await?;
	deleted.ok_or(DBerror::EventNotFound(event.clone()))
}

///deletes a category
pub async fn delete_category(db: Surreal<Client>, category: &Uuid) -> Result<Category, DBerror> {
	let deleted: Option<Category> = db.delete(("categories", category.to_raw())).await?;
	deleted.ok_or(DBerror::CategoryNotFound(category.clone()))
}
