use leptos::prelude::*;

/// Renders the Bitcoin page
#[component]
pub fn BitcoinPage() -> impl IntoView {
    view! {
        <h1>"Bitcoin Page"</h1>
        <p>"Welcome to the Bitcoin information page!"</p>
        <a href="/">"Back to Home"</a>
    }
}
