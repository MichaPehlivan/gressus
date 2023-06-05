cfg_if::cfg_if! {
	if #[cfg(feature = "ssr")] {
		use crate::backend::database::{db_error::DBResultConvert, db_requests};
		use crate::app::DB;
	}
}

use leptos::*;
use chrono::DateTime;
use chrono::Utc;
use surrealdb::sql::Uuid;
use crate::common::model::*;

#[server(UserIdFromName, "/api")]
pub async fn user_id_from_name(name: String) -> Result<Uuid, ServerFnError> {
	db_requests::user_id_from_name(&DB, &name)
		.await
		.to_server_error()
}

#[server(AddEvent, "/api")]
pub async fn add_event(
	name: String,
	description: String,
	start: DateTime<Utc>,
	end: DateTime<Utc>,
	category: Uuid,
	user: Uuid,
) -> Result<Event, ServerFnError> {
	db_requests::add_event(
		&DB,
		&name,
		&description,
		&start.into(),
		&end.into(),
		&category,
		&user,
	)
	.await
	.to_server_error()
}

// macro_rules! public_api {
// 	{$fn:ident($($arg:ident:$type:ty),*) -> $ret:ty} => {
// 		pub async fn $fn($($arg:$type),*) -> Result<$ret:ty, ServerFnError> {
// 			use crate::app::DB;
// 			db_requests::$fn(&DB, $($arg.into()),*);
// 		}
// 	}
// }

// #[server(UserIdFromName, "/api")]
// public_api! {user_id_from_name(name: String) -> Result<Uuid, ServerFnError>}
