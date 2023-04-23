use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use time::*;

use crate::frontend::*;
use overlay::*;
use pages::month::*;
use pages::day::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
	// Provides context that manages stylesheets, titles, meta tags, etc.
	provide_meta_context(cx);

	view! {
		cx,

		// injects a stylesheet into the document <head>
		// id=leptos means cargo-leptos will hot-reload this stylesheet
		<Stylesheet id="leptos" href="/pkg/gressus.css"/>

		// sets the document title
		<Title text="Gressus - agenda"/>

		// content for this welcome page
		<Router>
			<main>
				<Overlay>
					<Routes>
						<Route path="/month" view=|cx| view! { cx, <MonthView year=2023 month={Month::January}/> }/>
						<Route path="/day" view=|cx| view!{cx, <DayView date={Date::from_calendar_date(2022, Month::December, 1).unwrap()} />}/>
					</Routes>
				</Overlay>
			</main>
		</Router>
	}
}
