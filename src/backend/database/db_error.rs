use leptos::ServerFnError;
use surrealdb::sql::Uuid;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DBerror {
	#[error("database error: {0}")]
	SurrealError(#[from] surrealdb::Error),
	#[error("user with username {0} already exists")]
	UserAlreadyExists(String),
	#[error("no user found with username {0}")]
	UserNameNotFound(String),
	#[error("no user found with uuid {0}")]
	UserNotFound(Uuid),
	#[error("no task found with uuid {0}")]
	TaskNotFound(Uuid),
	#[error("no event found with uuid {0}")]
	EventNotFound(Uuid),
	#[error("no category found with uuid {0}")]
	CategoryNotFound(Uuid),
}

pub trait DBResultConvert<T> {
	fn to_server_error(self) -> Result<T, ServerFnError>
	where
		Self: Sized;
}

impl<T> DBResultConvert<T> for Result<T, DBerror> {
	#[inline(always)]
	fn to_server_error(self) -> Result<T, ServerFnError>
	where
		Self: Sized,
	{
		self.map_err(|e| e.into())
	}
}

impl From<DBerror> for ServerFnError {
	fn from(val: DBerror) -> Self {
		ServerFnError::ServerError(val.to_string())
	}
}
