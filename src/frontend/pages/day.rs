use chrono::prelude::*;
use leptos::*;
use surrealdb::sql::Datetime;

use crate::{
	common::{
		api,
		model::{Event, Timespan},
	},
	frontend::pages::get_day_events,
};

const SECONDS_PER_DAY: i64 = 24 * 60 * 60;
const SECONDS_PER_ROW: i64 = 15 * 60;
const NUM_ROWS: i64 = SECONDS_PER_DAY / SECONDS_PER_ROW;

#[component]
pub fn DayView(cx: Scope, date: NaiveDate) -> impl IntoView {
	async fn get_events((name, date): (String, NaiveDate)) -> Vec<Event> {
		let id = api::user_id_from_name(name).await.unwrap();
		let events = get_day_events(id, date).await.unwrap();
		events
	}

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
		<div class="dayview" style="grid-template-rows: repeat({NUM_ROWS}, 1fr);">
			<Suspense fallback=move || view! {cx, <p>"Loading..."</p>}>
				{events_view}
			</Suspense>
		</div>
	}
}

#[component]
pub fn DayEvent(cx: Scope, event: Event) -> impl IntoView {
	dbg!(&event);
	let Timespan {
		start: Datetime(start),
		end: Datetime(end),
	} = event.timespan;

	let start_time = start.time().num_seconds_from_midnight() as i64;
	let duration = (end - start).num_seconds();
	let end_time = (start_time + duration).min(SECONDS_PER_DAY);

	dbg!(start_time);
	dbg!(duration);
	dbg!(end_time);


	let start_row = start_time / SECONDS_PER_ROW;
	let end_row = end_time / SECONDS_PER_ROW;

	let title = event.name;

	view! {cx,
		<p class="dayview-item" style="grid-row-start: {start_row}; grid-row-end: {end_row};">{title}</p>
	}
}
