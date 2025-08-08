use leptos::prelude::*;

/// Renders the page header
#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header>
        <nav>
            <a href="/">"Home"</a>
            <a href="/bitcoin">"Bitcoin"</a>
            <a href="/ethereum">"Ethereum"</a>
            </nav>
        </header>
    }
}
