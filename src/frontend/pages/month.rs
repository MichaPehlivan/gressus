use crate::common;
// use crate::backend::database::db_requests;
use crate::common::model::*;
use chrono::{prelude::*, Days};
use leptos::*;
use surrealdb::sql::Uuid;

#[cfg(feature = "ssr")]
use crate::backend::database::{db_error::DBResultConvert, db_requests};

use super::get_day_events;

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

#[doc = "Returns all the events of a user in a month view."]
#[server(GetMonthEvents, "/api")]
pub async fn get_month_events(
	user_id: Uuid,
	year: i32,
	month: u8,
) -> Result<Vec<Event>, ServerFnError> {
	use crate::app::DB;
	let mut events = db_requests::get_events(&DB, &user_id)
		.await
		.to_server_error()?;

	let (start_of_view, end_of_view) = get_view_range(year, month.try_into().unwrap()).unwrap();
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

/// Renders a specific month's view.
#[component]
pub fn MonthView(
	cx: Scope,
	/// The year of the month to render.
	year: i32,
	/// The month number of the month to render.
	month: u32,
) -> impl IntoView {
	const ONE_DAY: Days = Days::new(1);

	let (current_date, _) = get_view_range(year, month).unwrap();
	let mut current_date = current_date.date();

	// This vec will be filled with views of an entire week/row.
	let mut weeks = Vec::with_capacity(5);

	for _rows in 0..WEEKS_IN_MONTH {
		// This vec will be filled with each day view of this week.
		let mut days_in_week = Vec::with_capacity(7);

		for _day in 0..7 {
			days_in_week.push(view! {cx, <Day date=current_date/>});
			current_date = current_date + ONE_DAY;
		}

		// Prepend the Days in the week with a week number, then push it into `weeks`.
		weeks.push(
			view! {cx, <p class="weeknumber">{current_date.iso_week().week()}</p> {days_in_week}}, //TODO: make config option to disable weeknumbers.
		);
	}

	view! {cx,
		<div class="monthview">
			<p>"Week"</p>
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

/// Renders a day of the month view.
#[component]
pub fn Day(
	cx: Scope,
	/// The date of this day view.
	date: NaiveDate,
) -> impl IntoView {
	// Retrieves the events of a user on a day.
	async fn get_events((name, date): (String, NaiveDate)) -> Vec<Event> {
		let id = common::api::user_id_from_name(name).await.unwrap();
		let events = get_day_events(id, date).await.unwrap();
		events
	} //TODO: write resource for retrieving uuid from name, then a vec of Events.

	let events = create_resource(cx, move || ("michah".into(), date), get_events); //TODO: change name

	let for_view = move |cx, event: Event| {
		view! {
			cx,
			<DayEvent event/>
		}
	};

	let events_view = move || {
		events.read(cx).map(|events| {
			view! {cx,
				<For
					each=move || {events.clone()}
					key=|event| event.uuid.clone()
					view=for_view
				/>
			}
		})
	};

	view! {cx,
		<div class="monthview-day">
			<p class="monthview-day-datum">{date.day()}</p>
			<div class="monthview-day-items-wrapper">
				<Suspense fallback=move || view! {cx, <p>"Loading..."</p>}>
					{events_view}
				</Suspense>
			</div>
		</div>
	}
}

#[component]
pub fn DayEvent(cx: Scope, event: Event) -> impl IntoView {
	let title = event.name;
	view! {cx,
		<p class="monthview-day-event" style=format!("background-color: #4a9cb3")>{title}</p>
	}
}
