use leptos::*;
use time::*;

#[component]
pub fn DayView(cx: Scope, date: Date) -> impl IntoView {
	let subdivide_by_minutes = 5;
	let num_rows = (24 * 60) / subdivide_by_minutes;

	let fill_items = (0..10)
		.into_iter()
		.map(|i| {
			let start = i * 20 + 1;
			let end = i * 20 + 2 + i*2;
			view! {cx,
				<p class="dayview-items" style="grid-row-start: {start}; grid-row-end: {end};">"TODO: {i}!"</p>
			}
		})
		.collect::<Vec<_>>();

	view! {cx,
		<div class="dayview" style="grid-template-rows: repeat({num_rows}, 1fr);">
			{fill_items}
		</div>
	}
}
