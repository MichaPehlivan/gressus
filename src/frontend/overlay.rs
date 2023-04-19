use leptos::*;
use leptos_router::*;

#[component]
pub fn Overlay(cx: Scope, children: Children) -> impl IntoView {
	view! {cx,
		<div class="overlay-wrapper">
			<Navbar/>
			{ children(cx) }
		</div>
	}
}

#[component]
pub fn Navbar(cx: Scope) -> impl IntoView {
	view!{cx,
		<div class="navbar">
			<div class="navbar-left">
				<p>"Fill text! jkljakflejalkfjeaklf"</p>
			</div>
			<div class="navbar-right">
				<A href="/user">
					<img id="usericon" src="/icons/user.svg"/>
				</A>
			</div>
		</div>
	}	
}