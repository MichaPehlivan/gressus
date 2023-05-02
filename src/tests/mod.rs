#[cfg(test)]
mod tests {
    use std::env;

    use surrealdb::{Surreal, engine::remote::ws::{Client, Ws}, opt::auth::Root};

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