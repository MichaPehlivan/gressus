use std::rc::Rc;

use chrono::prelude::*;
use leptos::*;
use leptos_router::{use_params, IntoParam, Params, ParamsError};
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

#[component]
pub fn DayView(cx: Scope, #[prop(optional)] date: Option<NaiveDate>) -> impl IntoView {
	let date = move || match date {
		Some(d) => d,
		None => use_params::<DayViewParams>(cx)().unwrap().date.0,
	};

	async fn get_events((name, date): (String, NaiveDate)) -> Vec<Event> {
		let id = api::user_id_from_name(name).await.unwrap();
		let events = get_day_events(id, date).await.unwrap();
		events
	}

	let events = create_resource(cx, move || ("michah".into(), date()), get_events); //TODO: change name

	let for_view = move |cx, event: Event| {
		view! {
			cx,
			<DayEvent event date={ date() }/>
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
		<div class="dayview" style="grid-template-rows: repeat({NUM_ROWS}, 1fr);">
			<Suspense fallback=move || view! {cx, <p>"Loading..."</p>}>
				{events_view}
			</Suspense>
		</div>
	}
}

#[component]
pub fn DayEvent(cx: Scope, event: Event, date: NaiveDate) -> impl IntoView {
	dbg!(&event);
	let Timespan {
		start: Datetime(start),
		end: Datetime(end),
	} = event.timespan;

	let (start_of_view, end_of_view) = get_day_view_range(date).unwrap();
	let start_of_view = start_of_view.and_local_timezone(Utc).unwrap();
	let end_of_view = end_of_view.and_local_timezone(Utc).unwrap();

	let start = start_of_view.max(start);
	let end = end_of_view.min(end);

	let start_secs = (start - start_of_view).num_seconds() as i64;
	let end_secs = (end - start_of_view).num_seconds() as i64;

	let start_row = (start_secs / SECONDS_PER_ROW).max(1);
	let end_row = (end_secs / SECONDS_PER_ROW).min(NUM_ROWS);

	let title = event.name;

	view! {cx,
		<p class="dayview-item" style="grid-row-start: {start_row}; grid-row-end: {end_row};">{title}</p>
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
			Err(e) => return Err(ParamsError::Params(Rc::new(e))),
		};
		Ok(Self(date))
	}
}

#[derive(Params, Clone, Copy, PartialEq, Debug)]
pub struct DayViewParams {
	pub date: NaiveDateWrapper,
}
