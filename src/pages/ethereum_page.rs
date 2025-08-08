use leptos::prelude::*;
use crate::components::Header;


#[component]
pub fn EthereumPage() -> impl IntoView {
    view! {
        <Header/>
        <h1>"Ethereum Page"</h1>
        <p>"Welcome to the Ethereum information page!"</p>
        <a href="/">"Back to Home"</a>
    }
}
