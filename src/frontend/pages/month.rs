use std::sync::Arc;

use crate::{common, frontend::pages::ViewError};
// use crate::backend::database::db_requests;
use crate::common::model::*;
use chrono::{prelude::*, Days};
use leptos::*;
use leptos_router::*;

// #[cfg(feature = "ssr")]
// use crate::backend::database::{db_error::DBResultConvert, db_requests};

use super::{get_day_events, get_month_events};

pub const WEEKS_IN_MONTH: usize = 6;
pub const DAYS_IN_MONTH: usize = WEEKS_IN_MONTH * 7;
pub const WEEK_START: Weekday = Weekday::Mon;

/// Returns the first and the last possible NaiveDateTime of this month's view.
pub fn get_first_of_view(year: i32, month: u8) -> Option<NaiveDateTime> {
	let first_of_month = NaiveDate::from_ymd_opt(year, month as u32, 1)?;
	// View in Dates
	let start_of_view = first_of_month.week(WEEK_START).first_day();
	// let end_of_view = start_of_view + Days::new(DAYS_IN_MONTH);
	// View in DateTimes
	let start_of_view = start_of_view.and_time(NaiveTime::from_hms_opt(0, 0, 0)?);
	// let end_of_view = end_of_view.and_time(NaiveTime::from_hms_opt(23, 59, 59)?);
	Some(start_of_view)
}

#[derive(Params, Clone, Copy, PartialEq, Debug)]
struct MonthViewParams {
	year: i32,
	month: u8,
}

/// Renders a specific month's view.
#[component]
pub fn MonthView(
	cx: Scope,
	/// The (year, month_number) of the view.
	#[prop(optional)]
	ym: Option<Signal<(i32, u8)>>,
) -> impl IntoView {
	let ym = match ym {
		Some(ym) => Signal::derive(cx, move || ym()),
		None => Signal::derive(cx, move || match use_params::<MonthViewParams>(cx)() {
			Ok(MonthViewParams { year, month }) => (year, month),
			Err(e) => (1, 1), //TODO: handle error
		}),
	};

	fn generate_empty() -> Vec<Option<Vec<Event>>> {
		(0..DAYS_IN_MONTH).into_iter().map(|_| None).collect::<_>()
	}

	// This function reads the resource and converts it into an array of options. The size of the array is precisely `DAYS_IN_MONTH`.
	async fn get_events((name, ym): (String, (i32, u8))) -> Vec<Option<Vec<Event>>> {
		async fn get_events_res(
			(name, ym): (String, (i32, u8)),
		) -> Result<Vec<Vec<Event>>, ServerFnError> {
			let id = common::api::user_id_from_name(name).await?;
			let events = get_month_events(id, ym).await;
			events
		}

		match get_events_res((name, ym)).await {
			Ok(ref events) => {
				events.clone().into_iter().map(|ev| Some(ev)).collect::<Vec<_>>()
			}
			Err(ref err) => {
				log!("{err}");
				generate_empty()
			}
		}
	}
	let events = create_resource(cx, move || ("michah".into(), ym()), get_events); // TODO: authentication

	let first_of_view = Signal::derive(cx, move || {
		let (year, month) = ym();
		get_first_of_view(year, month)
			.ok_or(ParamsError::Params(Arc::new(ViewError::OutOfRangeError)))
			.expect("Dates should be in the range of dates described by chrono.")
	});

	// Builds each reactive part of the month view.
	let mut weeks = Vec::with_capacity(WEEKS_IN_MONTH);
	let mut days_add = 0;
	for week_i in 0..WEEKS_IN_MONTH {
		let week_number = move || {
			(first_of_view().date() + Days::new(days_add + 1))
				.iso_week()
				.week()
		};

		// Prepares the reactive days for each week.
		let mut days_in_week = Vec::with_capacity(7);
		for day_i in 0..7 {
			let date = Signal::derive(cx, move || first_of_view().date() + Days::new(days_add));
			let index = days_add as usize;
			let day = view! {cx,
				<Day date events index/>
			};
			days_in_week.push(day);
			days_add += 1;
		}

		// Combines the reactive days of this week with a week number to obtain a week row.
		weeks.push(view! {cx,
			<p class="weeknumber">{week_number}</p>
			{days_in_week}
		});
	}

	// The reactive links to the previous and next months.
	let next_month = move || {
		let (year, month) = ym();
		let mut next_month = month + 1;
		let mut next_year = year;
		if next_month > 12 {
			next_month = 1;
			next_year += 1;
		}
		format!("/month/{next_year}/{next_month}")
	};
	let prev_month = move || {
		let (year, month) = ym();
		let mut prev_month = month - 1;
		let mut prev_year = year;
		if prev_month < 1 {
			prev_month = 12;
			prev_year -= 1;
		}
		format!("/month/{prev_year}/{prev_month}")
	};

	view! {cx,
		<A href=next_month>"Next month"</A>
		<A href=prev_month>"Previous month"</A>
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
	date: Signal<NaiveDate>,
	/// The events in this day view. Length should be [`DAYS_IN_MONTH`].
	events: Resource<(String, (i32, u8)), Vec<Option<Vec<Event>>>>,
	/// The index into the events resource that this event should use.
	index: usize,
) -> impl IntoView {
	// log!("Create_day");
	let day_view_link = move || date().format("/day/%Y-%m-%d").to_string();

	let display = move || {
		let events = events.with(cx, |ev| {
			ev[index].clone()
		}).flatten();
		events.map(|ev| {
			ev.into_iter()
				.map(|event| view! {cx, <DayEvent event/>})
				.collect::<Vec<_>>()
		})
	};

	view! {cx,
		<div class="monthview-day">
			<p class="monthview-day-datum">{move || date().day()}</p>
			<div class="monthview-day-items-wrapper">
			<Transition fallback=move || view!{cx, <p class="loading">"Loading..."</p>}>
				{display}
			</Transition>
			</div>
			<a href=day_view_link class="monthview-dayview-link reset-a">""</a>
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
