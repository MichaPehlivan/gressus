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
		Some(d) => d,
		None => Signal::derive(cx, move || match use_params::<DayViewParams>(cx)() {
			Ok(p) => p.date.0,
			Err(e) => NaiveDate::default(),
		}),
	};

	async fn get_events((name, date): (String, NaiveDate)) -> Result<Vec<Event>, ServerFnError> {
		let id = api::user_id_from_name(name).await?;
		let events = get_day_events(id, date).await?;
		Ok(events)
	}

	let events = create_resource(cx, move || ("michah".into(), date()), get_events); //TODO: change name

	let for_view = move |cx, event: Event| {
		log!("for_view");
		view! {
			cx,
			<DayEvent event date={Signal::derive(cx, move || date())}/> // Unwrap is allowed here, as `for_view` will not be called when there is an error.
		}
	};

	let events_view = move || {
		log!("events_view, pending_resources: {:?}", cx.pending_resources().len());
		events.with(cx, move |ev_result| {
			let ev_result = ev_result.clone();
			match ev_result {
				Ok(events) => view! {cx,
					<div class="dayview-item-container" style=move || format!("grid-template-rows: repeat({NUM_ROWS}, 1fr);")>
						{events.into_iter().map(move |event| for_view(cx, event)).collect::<Vec<_>>()}
					</div>
				},
				Err(_) => view! {cx,
					<div>
						<Redirect path="/notfound"/>
					</div>
				},
			}
		})
	};

	let next_date = move || (date() + ONE_DAY).format("/day/%Y-%m-%d").to_string();
	let prev_date = move || (date() - ONE_DAY).format("/day/%Y-%m-%d").to_string();

	view! {cx,
		<div class="dayview">
			<div class="dayview-navbar">
				<A href=next_date>"Next day"</A>
				<A href=prev_date>"Previous day"</A>
			</div>
				<Transition fallback=move || view! {cx, <p class="loading" style="grid-row: 1 / 20">"Loading..."</p>}>
					{ events_view }
				</Transition>
		</div>
	}
}

#[component]
pub fn DayEvent(cx: Scope, event: Event, date: Signal<NaiveDate>) -> impl IntoView {
	let Timespan {
		start: Datetime(start),
		end: Datetime(end),
	} = event.timespan;

	let row_style = Signal::derive(cx, move || { //TODO: This signal is not properly dropped after navigating to a new day, which causes the signals to accumulate and lag the page.
		log!("row_style");
		let (start_of_view, end_of_view) = get_day_view_range(date()).unwrap();
		let start_of_view = start_of_view.and_local_timezone(Utc).unwrap();
		let end_of_view = end_of_view.and_local_timezone(Utc).unwrap();

		let start = start_of_view.max(start);
		let end = end_of_view.min(end);

		let start_secs = (start - start_of_view).num_seconds() as i64;
		let end_secs = (end - start_of_view).num_seconds() as i64;

		if end_secs < start_secs {
			return "display: none".to_string();
		}

		let start_row = (start_secs / SECONDS_PER_ROW).max(1);
		let end_row = (end_secs / SECONDS_PER_ROW).min(NUM_ROWS);

		format!("grid-row-start: {start_row}; grid-row-end: {end_row};")
	});

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
