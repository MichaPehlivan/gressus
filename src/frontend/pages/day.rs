use std::sync::Arc;

use chrono::{prelude::*, Days};
use leptos::*;
use leptos_router::*;
use surrealdb::sql::Datetime;

use crate::{
	common::{
		api,
		model::{Event, Timespan},
	},
	frontend::pages::get_day_events,
};

use super::get_day_view_range;

const SECONDS_PER_DAY: i64 = 24 * 60 * 60;
const SECONDS_PER_ROW: i64 = 15 * 60;
const NUM_ROWS: i64 = SECONDS_PER_DAY / SECONDS_PER_ROW;
const ONE_DAY: Days = Days::new(1);

#[component]
pub fn DayView(cx: Scope, #[prop(optional)] date: Option<Signal<NaiveDate>>) -> impl IntoView {
	let date = match date {
		Some(d) => Signal::derive(cx, move || Ok(d())),
		None => create_memo(cx, move |_| {
			use_params::<DayViewParams>(cx)().map(|v| v.date.0)
		})
		.into(),
	}; // Signal with error, as date parsing may go wrong.

	// Accepts an Option<NaiveDate> as we have to bubble a possible parse error up.
	async fn get_events((name, date): (String, Option<NaiveDate>)) -> Option<Vec<Event>> {
		let id = api::user_id_from_name(name).await.unwrap();
		let events = get_day_events(id, date?).await.unwrap();
		Some(events)
	}

	let events = create_resource(cx, move || ("michah".into(), date().ok()), get_events); //TODO: change name

	let for_view = move |cx, event: Event| {
		view! {
			cx,
			<DayEvent event date={Signal::derive(cx, move || date().unwrap())}/> // Unwrap is allowed here, as `for_view` will not be called when there is an error.
		}
	};

	let events_view = move || {
		events.read(cx).map(|opt| match opt {
			Some(events) => view! {cx,
				<div class="dayview-item-container" style=move || format!("grid-template-rows: repeat({NUM_ROWS}, 1fr);")>
					<For
						each=move || {events.clone()}
						key=|event| event.uuid.clone()
						view=for_view
					/>
				</div>
			},
			None => view! {cx,
				<div>
					<Redirect path="/notfound"/>
				</div>
			},
		})
	};

	let next_date = move || match date() {
		Ok(d) => (d + ONE_DAY).format("/day/%Y-%m-%d").to_string(),
		Err(_) => "/notfound".to_string(), // If the current day is not valid, the next day is not either.
	};
	let prev_date = move || match date() {
		Ok(d) => (d - ONE_DAY).format("/day/%Y-%m-%d").to_string(),
		Err(_) => "/notfound".to_string(), // If the current day is not valid, the previous day is not either.
	};

	view! {cx,
		<div class="dayview">
			<div class="dayview-navbar">
				<A href=next_date>"Next day"</A>
				<A href=prev_date>"Previous day"</A>
			</div>
				<Suspense fallback=move || view! {cx, <p class="loading" style="grid-row: 1 / 20">"Loading..."</p>}>
					{ events_view }
				</Suspense>
		</div>
	}
}

#[component]
pub fn DayEvent(cx: Scope, event: Event, date: Signal<NaiveDate>) -> impl IntoView {
	dbg!(&event);
	let Timespan {
		start: Datetime(start),
		end: Datetime(end),
	} = event.timespan;

	let row_style = move || {
		let (start_of_view, end_of_view) = get_day_view_range(date()).unwrap();
		let start_of_view = start_of_view.and_local_timezone(Utc).unwrap();
		let end_of_view = end_of_view.and_local_timezone(Utc).unwrap();

		let start = start_of_view.max(start);
		let end = end_of_view.min(end);

		let start_secs = (start - start_of_view).num_seconds() as i64;
		let end_secs = (end - start_of_view).num_seconds() as i64;

		let start_row = (start_secs / SECONDS_PER_ROW).max(1);
		let end_row = (end_secs / SECONDS_PER_ROW).min(NUM_ROWS);

		format!("grid-row-start: {start_row}; grid-row-end: {end_row};")
	};

	let title = event.name;

	view! {cx,
		<p class="dayview-item" style=row_style>{title}</p>
	}
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct NaiveDateWrapper(pub NaiveDate);

impl IntoParam for NaiveDateWrapper {
	fn into_param(value: Option<&str>, name: &str) -> Result<Self, leptos_router::ParamsError> {
		let Some(value) = value else {
			return Err(leptos_router::ParamsError::MissingParam(name.to_string()));
		};
		let parse = value.parse::<NaiveDate>();
		let date = match parse {
			Ok(date) => date,
			Err(e) => return Err(ParamsError::Params(Arc::new(e))),
		};
		Ok(Self(date))
	}
}

#[derive(Params, Clone, Copy, PartialEq, Debug)]
pub struct DayViewParams {
	pub date: NaiveDateWrapper,
}
