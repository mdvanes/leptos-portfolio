use crate::api::get_rates;
use crate::components::{Header, Balance};
use leptos::prelude::*;

/// Renders the Bitcoin page
#[component]
pub fn BitcoinPage() -> impl IntoView {
    let rates_resource = Resource::new(|| (), |_| async move { get_rates().await });

    view! {
        <Header/>
        <h1>"Bitcoin Page"</h1>
        <p>"Welcome to the Bitcoin information page!"</p>
        <div>
            <Suspense fallback=move || view! { <p>"Loading prices..."</p> }>
                {move || {
                    rates_resource.get().map(|result| {
                        match result {
                            Ok(rates_response) => {
                                view! {
                                    // Example for mapping all products:
                                    // <div>
                                    //     <h3>"Rates:"</h3>
                                    //     {rates_response.products.into_iter().enumerate().map(|(i, rate)| {
                                    //         view! {
                                    //             <p>"Rate " {i + 1} ": €" {rate.price}</p>
                                    //         }
                                    //     }).collect::<Vec<_>>()}
                                    // </div>

                                    // Example for showing the first product:
                                    // <div>
                                    //     {rates_response.products.first().map(|rate| {
                                    //         view! {
                                    //             <p>"1 BTC is €" {rate.price}</p>
                                    //         }
                                    //     })}
                                    // </div>

                                    // With plain array of values:
                                    <div>
                                        {rates_response.first().map(|rate| {
                                            view! {
                                                <p>"1 BTC is €" {*rate}</p>
                                            }
                                        })}
                                    </div>
                                }.into_any()
                            }
                            Err(err) => {
                                let error_msg = err.to_string();
                                view! {
                                    <div>
                                        <h3>"Error:"</h3>
                                        {vec![view! { <p style="color: red;">"Error loading BTC rate: " {error_msg}</p> }]}
                                    </div>
                                }.into_any()
                            }
                        }
                    })
                }}
            </Suspense>
        </div>

        <Balance/>

        // <a href="/">"Back to Home"</a>
    }
}
