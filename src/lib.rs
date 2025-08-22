pub mod api;
pub mod app;
pub mod components;
pub mod pages;
pub mod utils;
#[cfg(feature = "ssr")]
pub mod storage;
#[cfg(feature = "ssr")]
pub mod websocket;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
