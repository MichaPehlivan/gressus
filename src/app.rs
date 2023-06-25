use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::frontend::*;
use overlay::*;
use pages::day::*;
use pages::month::*;

cfg_if::cfg_if! {
	if #[cfg(feature = "ssr")] {

		pub fn register_server_fns() {
			// _ = pages::month::GetMonthEvents::register();

			use crate::common::api::*;
			_ = pages::GetDayEvents::register();
			_ = pages::GetMonthEvents::register();
			_ = UserIdFromName::register();
			_ = AddEvent::register();
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
		<Stylesheet id="leptos" href="\\pkg\\gressus.css"/>

		// sets the document title
		<Title text="Gressus - agenda"/>

		// content for this welcome page
		<Router>
			<main>
				<Overlay>
					<Routes>
						<Route path="/notfound" view=|cx| view! {cx, <Redirect path="/month/2023/5"/>}/>
						<Route path="/month/:year/:month" view=|cx| view! { cx, <MonthView/> }/>
						<Route path="/day/:date" view=|cx| view!{cx, <DayView/>}/>
					</Routes>
				</Overlay>
			</main>
		</Router>
	}
}
