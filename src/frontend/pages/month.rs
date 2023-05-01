use crate::backend::database::db_requests;
use crate::common::model::*;
use chrono::{prelude::*, Days};
use leptos::*;
use surrealdb::sql::Uuid;

const WEEKS_IN_MONTH: u64 = 6;
const WEEK_START: Weekday = Weekday::Mon;

/// Returns the first and the last possible NaiveDateTime of this month's view.
pub fn get_view_range(year: i32, month: u32) -> Option<(NaiveDateTime, NaiveDateTime)> {
	let first_of_month = NaiveDate::from_ymd_opt(year, month, 1)?;
	// View in Dates
	let start_of_view = first_of_month.week(WEEK_START).first_day();
	let end_of_view = start_of_view + Days::new(WEEKS_IN_MONTH * 7);
	// View in DateTimes
	let start_of_view = start_of_view.and_time(NaiveTime::from_hms_opt(0, 0, 0)?);
	let end_of_view = end_of_view.and_time(NaiveTime::from_hms_opt(23, 59, 59)?);
	Some((start_of_view, end_of_view))
}

/// Returns all the events of a user in a month view.
#[server(GetMonthEvents, "/api")]
pub async fn get_month_events(
	user_id: Uuid,
	year: i32,
	month: u8,
) -> Result<Vec<Event>, ServerFnError> {
	use crate::app::DB;
	let mut events = db_requests::get_events(&DB, &user_id).await;

	let (start_of_view, end_of_view) = get_view_range(year, month.try_into().unwrap()).unwrap();
	// Convert the range into timezone-aware DateTimes
	let start_of_view = Utc.from_local_datetime(&start_of_view).unwrap();
	let end_of_view = Utc.from_local_datetime(&end_of_view).unwrap();
	// Filter for the events in the view range
	let events_in_view = events.drain_filter(|e| {
		e.timespan.end >= start_of_view.into() && e.timespan.start <= end_of_view.into()
	}).collect::<Vec<_>>();
	Ok(events_in_view)
}

#[component]
pub fn MonthView(cx: Scope, year: i32, month: u32) -> impl IntoView {

	const ONE_DAY: Days = Days::new(1);

	let (current_date, _) = get_view_range(year, month).unwrap();
	let mut current_date = current_date.date();
	let mut weeks = Vec::with_capacity(5); // The vec with the weeks/rows
	// Iterate over each week in the month view
	for _rows in 0..WEEKS_IN_MONTH {
		// Fill a vec with all the Days in the week
		let mut days_in_week = Vec::with_capacity(7);
		for date in 0..7 {
			days_in_week.push(view! {cx, <Day date=current_date/>});
			current_date = current_date + ONE_DAY;
		}
		// weeks.push(view!{cx, <p class="empty"></p> {days_in_week}}); // Uncomment to disable week numbers. TODO: make config option.

		// Prepend the Days in the week with a week number.
		weeks.push(
			view! {cx, <p class="weeknumber">{current_date.iso_week().week()}</p> {days_in_week}},
		);
		// Comment to disable week numbers.
	}

	view! {cx,
		<div class="monthview">
			<p>"Week"</p> // Comment to disable week numbers.
			// <p></p> // Uncomment to disable week numbers. TODO: make config option.
			<p>"Mon"</p>
			<p>"Tue"</p>
			<p>"Wed"</p>
			<p>"Thi"</p>
			<p>"Fri"</p>
			<p>"Sat"</p>
			<p>"Sun"</p>
			{weeks}
		</div>
	}
}

#[component]
pub fn Day(cx: Scope, date: NaiveDate) -> impl IntoView {
	let items_fill = (0..5)
		.into_iter()
		.map(
			|n| view! {cx, <DayEvent description={ format!("TODO: {n}")} color="#1E70F0".to_string()/> },
		)
		.collect::<Vec<_>>();
	
	

	view! {cx,
		<div class="monthview-day">
			<p class="monthview-day-datum">{date.day()}</p>
			<div class="monthview-day-items-wrapper">
				{items_fill}
			</div>
		</div>
	}
}

#[component]
pub fn DayEvent(cx: Scope, description: String, color: String) -> impl IntoView {
	view! {cx,
		<p class="monthview-day-event" style=format!("background-color: {color}")>{description}</p>
	}
}
