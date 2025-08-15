use crate::api::get_rates;
use crate::components::{Balance, Header};
use leptos::prelude::*;

/// Renders the Bitcoin page
#[component]
pub fn BitcoinPage() -> impl IntoView {
    let rates_resource = LocalResource::new(|| async move { get_rates().await });

    // Create a memo that gets the first rate from the rates response
    let newRate = Memo::new(move |_| {
        rates_resource.get().and_then(|result| match result {
            Ok(rates_response) => rates_response.first().copied(),
            Err(_) => None,
        })
    });

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
                                    // <div>
                                    //     {move || {
                                    //         rate1.get().map(|rate| {
                                    //             view! {
                                    //                 <p>"1 BTC is €" {rate}</p>
                                    //             }
                                    //         })
                                    //     }}
                                    // </div>
                                    <div>
                                        {rates_response.first().map(|rate| {
                                            view! {
                                                <p>"1 BTC is €" {*rate}</p>
                                            }
                                        })}
                                        // <p>"1 BTC is €" {rate1}</p>
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

        <Balance rate=newRate/>

        // <a href="/">"Back to Home"</a>
    }
}
