#![feature(drain_filter)]

pub mod app;
pub mod backend;
pub mod common;
pub mod frontend;

use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "hydrate")] {

  use wasm_bindgen::prelude::wasm_bindgen;

	#[wasm_bindgen]
	pub fn hydrate() {
	  use app::*;
	  use leptos::*;

	  // initializes logging using the `log` crate
	  _ = console_log::init_with_level(log::Level::Debug);
	  console_error_panic_hook::set_once();

	  leptos::mount_to_body(move |cx| {
		  view! { cx, <App/> }
	  });
	}
}
}
