use crate::api::get_rates;
use crate::components::Header;
use leptos::prelude::*;

#[component]
pub fn EthereumPage() -> impl IntoView {
    let rates_resource = Resource::new(|| (), |_| async move { get_rates().await });

    view! {
        <Header/>
        <h1>"Ethereum Page"</h1>
        <p>"Welcome to the Ethereum information page!"</p>
        <div>
            <Suspense fallback=move || view! { <p>"Loading ETH price..."</p> }>
                {move || {
                    rates_resource.get().map(|result| {
                        match result {
                            Ok(rates_response) => {
                                view! {
                                    <div>
                                        {rates_response.get(1).map(|rate| {
                                            view! {
                                                <p>"1 ETH is €" {*rate}</p>
                                            }
                                        })}
                                    </div>
                                }.into_any()
                            }
                            Err(err) => {
                                let error_msg = err.to_string();
                                view! {
                                    <div>
                                        <p style="color: red;">"Error loading ETH rate: " {error_msg}</p>
                                    </div>
                                }.into_any()
                            }
                        }
                    })
                }}
            </Suspense>
        </div>
        <a href="/">"Back to Home"</a>
    }
}
