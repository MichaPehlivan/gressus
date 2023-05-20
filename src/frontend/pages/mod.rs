pub mod day;
pub mod home;
pub mod login;
pub mod month;

use chrono::{prelude::*, Days};
use leptos::*;
use surrealdb::sql;
use surrealdb::sql::Uuid;
use thiserror::Error;

use crate::common::model::{Event, Timespan};

#[derive(Error, Debug)]
pub enum ViewError {
	#[error("Date out of range.")]
	OutOfRangeError,
}

#[cfg(feature = "ssr")]
impl<T> DBResultConvert<T> for Result<T, ViewError> {
	fn to_server_error(self: Self) -> Result<T, ServerFnError>
	where
		Self: Sized,
	{
		self.map_err(|e| e.into())
	}
}

impl Into<ServerFnError> for ViewError {
	fn into(self) -> ServerFnError {
		ServerFnError::ServerError(self.to_string())
	}
}

/// Returns the first and the last possible NaiveDateTime of this day's view.
pub fn get_day_view_range(date: NaiveDate) -> Option<(NaiveDateTime, NaiveDateTime)> {
	let start_of_view = date.and_time(NaiveTime::from_hms_opt(0, 0, 0)?);
	let end_of_view = date.and_time(NaiveTime::from_hms_opt(23, 59, 59)?);
	Some((start_of_view, end_of_view))
}

#[cfg(feature = "ssr")]
use crate::app::DB;
#[cfg(feature = "ssr")]
use crate::backend::database::{db_error::DBResultConvert, db_requests};

#[server(GetDayEvents, "/api")]
pub async fn get_day_events(user_id: Uuid, date: NaiveDate) -> Result<Vec<Event>, ServerFnError> {
	let mut events = db_requests::get_events(&DB, &user_id)
		.await
		.to_server_error()?;

	let (start_of_view, end_of_view) = get_day_view_range(date).unwrap();

	// Convert the range into timezone-aware DateTimes
	let start_of_view = Utc.from_local_datetime(&start_of_view).unwrap();
	let end_of_view = Utc.from_local_datetime(&end_of_view).unwrap();

	// Filter for the events in the view range
	let events_in_view = events
		.drain_filter(|e| {
			e.timespan.end >= start_of_view.into() && e.timespan.start <= end_of_view.into()
		})
		.collect::<Vec<_>>();

	Ok(events_in_view)
}

/// Returns an array of events per day, given a user and a month.
#[server(GetMonthEvents, "/api")]
pub async fn get_month_events(
	user_id: Uuid,
	ym: (i32, u8),
) -> Result<Vec<Vec<Event>>, ServerFnError> {
	let (year, month) = ym;

	let events = db_requests::get_events(&DB, &user_id)
		.await
		.to_server_error()?;

	let start_of_view =
		month::get_first_of_view(year, month).ok_or(ViewError::OutOfRangeError.into())?;
	let end_of_view = start_of_view + Days::new(month::DAYS_IN_MONTH as u64);

	// Convert the range into timezone-aware DateTimes
	let start_of_view = Utc.from_local_datetime(&start_of_view).unwrap();
	let end_of_view = Utc.from_local_datetime(&end_of_view).unwrap();

	let mut sorted_events: Vec<Vec<Event>> = vec![Vec::new(); 42];

	// For each event, determine if it is a member of the month view, and then which day view it is a member of.
	for e in events {
		let Timespan {
			start: sql::Datetime(start),
			end: sql::Datetime(end),
		} = e.timespan;
		if end < start_of_view || start > end_of_view {
			continue;
		}
		let days_start = (start - start_of_view).num_days();
		let days_end = (end - start_of_view).num_days();
		for i in days_start..=days_end {
			sorted_events[i as usize].push(e.clone());
		}
	}

	Ok(sorted_events)
}
