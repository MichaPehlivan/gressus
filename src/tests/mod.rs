#[cfg(test)]
mod tests {
    use std::env;

    use chrono::Utc;
    use surrealdb::{Surreal, engine::remote::ws::{Client, Ws}, opt::auth::Root, sql::Uuid};
    use crate::{backend::database::db_requests::{add_event, add_user, user_id_from_name, add_task, get_tasks, get_events, change_username, delete_user, add_category, get_user, get_category, get_task, get_event}, common::model::{Category, Event, User, Task, Timespan}};

    async fn setup() -> Surreal<Client> {
        let db = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();

        // Signin to database
        db.signin(Root {
            username: &env::var("DATABASE_USER").unwrap(),
            password: &env::var("DATABASE_PASS").unwrap(),
        })
        .await.unwrap();

        // Select a specific namespace / database
        db.use_ns("main").use_db("main").await.unwrap();
        db
    }

    #[tokio::test]
    async fn test_create() {
        let db = setup().await;
        let time = Utc::now();
        let user = add_user(&db, "micha", &"pass".as_bytes().to_vec()).await.unwrap().uuid;
        let category = add_category(&db, "category1", 0, &user).await.unwrap().uuid;
        let task = add_task(&db, "test_task", "task_description", &time, &time, &category, &user).await.unwrap().uuid;
        let event = add_event(&db, "test_event", "event_description", &time, &time, &category, &user).await.unwrap().uuid;

        let user_test = User {
            name: "micha".to_string(),
            hashed_password: "pass".as_bytes().to_vec(),
            joined_at: get_user(&db, &user).await.unwrap().joined_at,
            categories: vec![category.clone()],
            uuid: user.clone(),
        };

        let category_test = Category {
            name: "category1".to_string(),
            color: 0,
            user: user.clone(),
            uuid: category.clone(),
        };

        let task_test = Task {
            name: "test_task".to_string(),
            description: "task_description".to_string(),
            timespan: Timespan::new(&time, &time),
            category: category.clone(),
            completed: false,
            user: user.clone(),
            uuid: task.clone(),
        };

        let event_test = Event {
            name: "test_event".to_string(),
            description: "event_description".to_string(),
            timespan: Timespan::new(&time, &time),
            category: category.clone(),
            user: user.clone(),
            uuid: event.clone(),
        };

        assert_eq!(get_user(&db, &user).await.unwrap(), user_test);
        assert_eq!(get_category(&db, &category).await.unwrap(), category_test);
        assert_eq!(get_task(&db, &task).await.unwrap(), task_test);
        assert_eq!(get_event(&db, &event).await.unwrap(), event_test);
    } 

    #[tokio::test]
    async fn test_update() {
        let db = setup().await;
    }

    #[tokio::test]
    async fn test_delete() {
        let db = setup().await;
    }
}