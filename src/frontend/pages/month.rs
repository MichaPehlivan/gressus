use crate::backend::database::db_requests;
use crate::common::model::*;
use chrono::{prelude::*, Days};
use leptos::*;
use surrealdb::sql::Uuid;

const WEEKS_IN_MONTH: u64 = 6;
const WEEK_START: Weekday = Weekday::Mon;

pub fn get_view_range(year: i32, month: u32) -> Option<(NaiveDate, NaiveDate)> {
	// Get the DateTime of the first day of the month...
	let first_of_month = NaiveDate::from_ymd_opt(year, month, 1)?;
	// ...and the Date of the first day of the week...
	let first_of_view = first_of_month.week(WEEK_START).first_day();
	let last_of_view = first_of_view + Days::new(WEEKS_IN_MONTH * 7);
	Some((first_of_view, last_of_view))
}

#[server(GetMonthItems, "/api")]
pub async fn get_month_events(
	user_id: Uuid,
	year: i32,
	month: u8,
) -> Result<Vec<Event>, ServerFnError> {
	use crate::app::DB;
	let mut events = db_requests::get_events(&DB, &user_id).await;
	let (first_of_month, last_of_month) = get_view_range(year, month.try_into().unwrap()).unwrap();
	// let events_in_view = events.drain_filter(|e| {(e.timespan.end >= first_of_month)});
	todo!();
}

#[component]
pub fn MonthView(cx: Scope, year: i32, month: u32) -> impl IntoView {
	// ...such that we can now fill a vec with 35 dates, starting from the first of the week.
	const ONE_DAY: Days = Days::new(1);
	let (mut current_date, _) = get_view_range(year, month).unwrap();
	let mut weeks = Vec::with_capacity(5);
	for _rows in 0..WEEKS_IN_MONTH {
		let mut days_in_week = Vec::with_capacity(7);
		for date in 0..7 {
			days_in_week.push(view! {cx, <Day date=current_date/>});
			current_date = current_date + ONE_DAY;
		}
		// weeks.push(view!{cx, <p class="empty"></p> {days_in_week}}); // Uncomment to disable week numbers. TODO: make config option.
		weeks.push(view! {cx, <p class="weeknumber">{current_date.iso_week().week()}</p> {days_in_week}});
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
