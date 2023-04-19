use leptos::*;
use time::*;

#[component]
pub fn MonthView(cx: Scope, year: i32, month: Month) -> impl IntoView {
	// let days_fill = (-6..=42)
	// 	.into_iter()
	// 	.map(|n| {
	// 		view! {cx,
	// 			<Day day=n/>
	// 		}
	// 	})
	// 	.collect::<Vec<_>>();

	// Get the Date of the first day of the month...
	let first_of_month = Date::from_calendar_date(year, month, 1).unwrap();
	// ...and the Date of the first day of the week...
	let (_, week, _) = first_of_month.to_iso_week_date();
	let first_of_week = Date::from_iso_week_date(year, week, Weekday::Monday).unwrap();
	// ...such that we can now fill a vec with all the dates aligned to the weekdays until we reach the first of the month...
	let mut dates = Vec::with_capacity(42);
	let mut current_date = first_of_week;
	while current_date != first_of_month {
		dates.push(current_date);
		current_date = current_date.next_day().unwrap();
	}
	// ...and then fill it until we reach the next month...
	while current_date.month() == first_of_month.month() {
		dates.push(current_date);
		current_date = current_date.next_day().unwrap();
	}
	// ...and then until we reach the start of next week.
	let (year, week, _) = current_date.to_iso_week_date();
	let last_of_week = Date::from_iso_week_date(year, week, Weekday::Sunday).unwrap().next_day().unwrap();
	dbg!(current_date);
	dbg!(last_of_week);
	while current_date != last_of_week {
		dates.push(current_date);
		current_date = current_date.next_day().unwrap();
	}

	let days = dates
		.iter()
		.map(|d| view! {cx, <Day date=*d/>})
		.collect::<Vec<_>>();

	view! {cx,
		<div class="month-view">
			<p>"Mon"</p>
			<p>"Tue"</p>
			<p>"Wed"</p>
			<p>"Thi"</p>
			<p>"Fri"</p>
			<p>"Sat"</p>
			<p>"Sun"</p>
			{days}
		</div>
	}
}

#[component]
pub fn Day(cx: Scope, date: Date) -> impl IntoView {
	let items_fill = (0..5)
		.into_iter()
		.map(
			|n| view! {cx, <DayEvent description={ format!("Fill text {n} feafea!!")} color="#1E70F0".to_string()/> },
		)
		.collect::<Vec<_>>();
	view! {cx,
		<div class="month-day">
			<p class="month-day-datum">{date.day()}</p>
			<div class="month-day-items-wrapper">
				{items_fill}
			</div>
		</div>
	}
}

#[component]
pub fn DayEvent(cx: Scope, description: String, color: String) -> impl IntoView {
	view! {cx,
		<p class="month-day-event" style=format!("background-color: {color}")>{description}</p>
	}
}
