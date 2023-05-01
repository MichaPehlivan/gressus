use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use chrono::prelude::*;

use crate::frontend::*;
use overlay::*;
use pages::month::*;
use pages::day::*;
use surrealdb::Surreal;

cfg_if::cfg_if! {
	if #[cfg(feature = "ssr")] {
		/// The surrealdb singleton
		pub static DB: Surreal<surrealdb::engine::remote::ws::Client> = Surreal::init();

		pub fn register_server_fns() {
			_ = pages::month::GetMonthItems::register();
		}
	}
}

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
						<Route path="/month" view=|cx| view! { cx, <MonthView year=2023 month=5/> }/>
						<Route path="/day" view=|cx| view!{cx, <DayView date={NaiveDate::from_ymd_opt(2023, 5, 1).unwrap()} />}/>
					</Routes>
				</Overlay>
			</main>
		</Router>
	}
}
