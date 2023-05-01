use leptos::*;
use chrono::prelude::*;

#[component]
pub fn DayView(cx: Scope, date: NaiveDate) -> impl IntoView {
	let subdivide_by_minutes = 15;
	let num_rows = (24 * 60) / subdivide_by_minutes;

	let mut fill_items = Vec::with_capacity(10);
	let mut start = 0;
	for i in 1..11 {
		let end = start + i;
		fill_items.push(view! {cx,
			<p class="dayview-item" style="grid-row-start: {start}; grid-row-end: {end};">"TODO: {i}!"</p>
		});
		start += i + 1;
	}

	view! {cx,
		<div class="dayview" style="grid-template-rows: repeat({num_rows}, 1fr);">
			{fill_items}
		</div>
	}
}

#[component]
pub fn DayItem(cx: Scope, start: NaiveDate) -> impl IntoView {
	
	view!{cx,
		
	}
}