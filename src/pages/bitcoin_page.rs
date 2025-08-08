use leptos::prelude::*;
use crate::components::Header;


/// Renders the Bitcoin page
#[component]
pub fn BitcoinPage() -> impl IntoView {
    view! {
        <Header/>
        <h1>"Bitcoin Page"</h1>
        <p>"Welcome to the Bitcoin information page!"</p>
        <a href="/">"Back to Home"</a>
    }
}
