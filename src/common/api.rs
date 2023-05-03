#[cfg(feature = "ssr")]
use crate::backend::database::{db_error::DBResultConvert, db_requests};

use leptos::*;
use surrealdb::sql::Uuid;

#[server(UserIdFromName, "/api")]
pub async fn user_id_from_name(name: String) -> Result<Uuid, ServerFnError> {
	use crate::app::DB;
	db_requests::user_id_from_name(&DB, &name)
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
