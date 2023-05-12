
use std::sync::Arc;

use crate::{common, frontend::pages::ViewError};
// use crate::backend::database::db_requests;
use crate::common::model::*;
use chrono::{prelude::*, Days};
use leptos::*;
use leptos_router::*;

// #[cfg(feature = "ssr")]
// use crate::backend::database::{db_error::DBResultConvert, db_requests};

use super::get_day_events;

const WEEKS_IN_MONTH: u64 = 6;
const WEEK_START: Weekday = Weekday::Mon;

/// Returns the first and the last possible NaiveDateTime of this month's view.
pub fn get_first_of_view(year: i32, month: u8) -> Option<NaiveDateTime> {
	let first_of_month = NaiveDate::from_ymd_opt(year, month as u32, 1)?;
	// View in Dates
	let start_of_view = first_of_month.week(WEEK_START).first_day();
	// let end_of_view = start_of_view + Days::new(WEEKS_IN_MONTH * 7);
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
	let ym = Signal::derive(cx, move || {
		Ok(match ym {
			Some(ym) => ym(),
			None => match use_params::<MonthViewParams>(cx)()? {
				MonthViewParams { year, month } => (year, month),
			},
		})
	});

	let first_of_view = create_memo(cx, move |_| {
		let (year, month) = ym()?;
		get_first_of_view(year, month)
			.ok_or(ParamsError::Params(Arc::new(ViewError::OutOfRangeError)))
	});

	let mut weeks = Vec::with_capacity(WEEKS_IN_MONTH as usize);
	let mut days_add = 0;
	for row in 0..WEEKS_IN_MONTH {
		let mut days_in_week = Vec::with_capacity(7);

		for day in 0..7 {
			let current_date = Signal::derive(cx, move || {
				first_of_view().unwrap().date() + Days::new(days_add)
			});
			days_in_week.push((day, current_date));
			// days_in_week.push(view! {cx, <Day date=current_date/>});
			days_add += 1;
		}

		weeks.push((row, days_in_week))

		// weeks.push(
		// 	view! {cx, <p class="weeknumber">{current_date.iso_week().week()}</p> {days_in_week}}, //TODO: make config option to disable weeknumbers.
		// );
	}

	let render_weeks = move |cx, week: (u64, Vec<(i32, Signal<NaiveDate>)>)| {
		let days = week.1;
		let first_of_week = days[0].1;
		let week_number = move || first_of_week().iso_week().week();
		let (foreach, _) = create_signal(cx, days);
		view! {cx,
			<p class="weeknumber">{week_number}</p>
			<For
				each=foreach
				key=|d| d.0
				view=move |cx, day| {
					// log!("Reloading week item.");
					let date = day.1;
					view! {cx,
						<Day date/>
					}
				}
			/>
		}
	};

	let (weeks, _) = create_signal(cx, weeks);

	let next_month = move || match ym() {
		Ok((year, month)) => {
			let mut next_month = month + 1;
			let mut next_year = year;
			if next_month > 12 {
				next_month = 1;
				next_year += 1;
			}
			format!("/month/{next_year}/{next_month}")
		}
		Err(_) => "/notfound".to_string(), // If the current month is not valid, the next month is not either.
	};
	let prev_month = move || match ym() {
		Ok((year, month)) => {
			let mut prev_month = month - 1;
			let mut prev_year = year;
			if prev_month < 1 {
				prev_month = 12;
				prev_year -= 1;
			}
			format!("/month/{prev_year}/{prev_month}")
		}
		Err(_) => "/notfound".to_string(), // If the current month is not valid, the previous month is not either.
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
			<For
				each=weeks
				key=|week| week.0
				view=render_weeks
			/>
		</div>
	}
}

/// Renders a day of the month view.
#[component]
pub fn Day(
	cx: Scope,
	/// The date of this day view.
	date: Signal<NaiveDate>,
) -> impl IntoView {
	// log!("Create_day");
	// Retrieves the events of a user on a day.
	async fn get_events((name, date): (String, NaiveDate)) -> Vec<Event> {
		let id = common::api::user_id_from_name(name).await.unwrap();
		let events = get_day_events(id, date).await.unwrap();
		events
	} //TODO: write resource for retrieving uuid from name, then a vec of Events.

	let events_resource = create_resource(cx, move || ("michah".into(), date()), get_events); //TODO: change name

	let (events, set_events) = create_signal(cx, Vec::<Event>::new());

	let events_view = Signal::derive(cx, move || {
		// log!("events_view!");
		events_resource.read(cx).map(|mut ev| {
			set_events.update(|v| {
				v.clear();
				v.append(&mut ev);
			});
		});
	});

	view! {cx,
			<div class="monthview-day">
				<p class="monthview-day-datum">{move || date().day()}</p>
				<div class="monthview-day-items-wrapper">
					<Transition fallback=move || view! {cx, <p>"Loading..."</p>}>
						{events_view}
						{move || events().into_iter().map(|event| view!{cx, <DayEvent event/>}).collect::<Vec<_>>()}
					</Transition>
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
