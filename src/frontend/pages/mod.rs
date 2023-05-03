pub mod day;
pub mod home;
pub mod month;
pub mod login;

use chrono::prelude::*;
use leptos::*;
use surrealdb::sql::Uuid;

use crate::common::model::Event;

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
