
#![recursion_limit = "256"]

pub mod app;
pub mod components;
pub mod placeholders;
pub mod views;
pub mod config;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
  use app::*;
  use wasm_bindgen_console_logger::DEFAULT_LOGGER;
  console_error_panic_hook::set_once();
  log::set_logger(&DEFAULT_LOGGER).unwrap();
  log::set_max_level(log::LevelFilter::Info);
  leptos::mount::hydrate_body(App);
}
