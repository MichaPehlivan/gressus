use leptos::*;

#[component]
pub fn MonthView(cx: Scope) -> impl IntoView {
	let a = (1..31)
		.into_iter()
		.map(|n| view! {cx,
			<MonthDay datum=n/>
		})
		.collect::<Vec<_>>();
	view! {cx,
		<div class="month-view">
			{a}
		</div>
	}
}

#[component]
pub fn MonthDay(cx: Scope, datum: i32) -> impl IntoView {
	view! {cx,
		<div class="month-day">
			<p class="month-day-datum">{datum}</p>
			<div class="month-day-items-wrapper">
			</div>
		</div>
	}
}
