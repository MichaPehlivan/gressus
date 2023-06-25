use std::{env, error::Error};

use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

pub mod db_error;
pub mod db_requests;

/// The surrealdb singleton
pub static DB: Surreal<surrealdb::engine::remote::ws::Client> = Surreal::init();

pub async fn init() -> Result<(), Box<dyn Error>> {
	DB.connect::<Ws>("127.0.0.1:8000").await?;

	DB.signin(Root {
		username: &env::var("DATABASE_USER").unwrap(),
		password: &env::var("DATABASE_PASS").unwrap(),
	})
	.await?;

	DB.query(
		"
		DEFINE NAMESPACE main;
		USE NS main;

		DEFINE DATABASE main;
		USE DB main;

		DEFINE TABLE users SCHEMALESS;

		DEFINE TABLE events SCHEMALESS
			PERMISSIONS
				FOR create, select, update, delete
					WHERE user = $auth.uuid
		;
	",
	)
	.await?;

	Ok(())
}