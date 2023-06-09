use leptos::*;
use time::*;

#[component]
pub fn MonthView(cx: Scope, year: i32, month: Month) -> impl IntoView {
	// Get the Date of the first day of the month...
	let first_of_month = Date::from_calendar_date(year, month, 1).unwrap();
	// ...and the Date of the first day of the week...
	let mut first_of_week = first_of_month;
	//TODO: maybe create a setting for the first day of the week.
	while first_of_week.weekday() != Weekday::Monday {
		first_of_week = first_of_week.previous_day().unwrap();
	}
	// ...such that we can now fill a vec with 35 dates, starting from the first of the week.
	let mut current_date = first_of_week;
	let mut weeks = Vec::with_capacity(5);
	for _rows in 0..6 {
		let mut days_in_week = Vec::with_capacity(7);
		for _days_in_row in 0..7 {
			days_in_week.push(view! {cx, <Day date=current_date/>});
			current_date = current_date.next_day().unwrap();
		}
		// weeks.push(view!{cx, <p class="empty"></p> {days_in_week}}); // Uncomment to disable week numbers. TODO: make config option.
		weeks.push(view! {cx, <p class="weeknumber">{current_date.iso_week()}</p> {days_in_week}});
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
pub fn Day(cx: Scope, date: Date) -> impl IntoView {
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
