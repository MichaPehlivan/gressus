use leptos::*;

#[component]
pub fn MonthView(cx: Scope) -> impl IntoView {
	let days_fill = (1..=31)
		.into_iter()
		.map(|n| view! {cx,
			<Day day=n/>
		})
		.collect::<Vec<_>>();
	view! {cx,
		<div class="month-view">
			{days_fill}
		</div>
	}
}

#[component]
pub fn Day(cx: Scope, day: u8) -> impl IntoView {
	let items_fill = (0..5).into_iter().map(|n| view!{cx, <DayEvent description={ format!("Fill text {n}")} color="#1E70F0".to_string()/> }).collect::<Vec<_>>();
	view! {cx,
		<div class="month-day">
			<p class="month-day-datum">{day}</p>
			<div class="month-day-items-wrapper">
				{items_fill}
			</div>
		</div>
	}
}

#[component]
pub fn DayEvent(cx: Scope, description: String, color: String) -> impl IntoView {
	view!{cx, 
		<p class="month-day-event" style=format!("background-color: {color}")>{description}</p>
	}
}