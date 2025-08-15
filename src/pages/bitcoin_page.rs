use crate::api::{get_rates, get_balance};
use crate::components::Header;
use crate::utils::get_formatted_now;
use leptos::prelude::*;

/// Renders the Bitcoin page
#[component]
pub fn BitcoinPage() -> impl IntoView {
    let rates_resource = Resource::new(|| (), |_| async move { get_rates().await });
    let add_transaction_to_balance = ServerAction::<crate::api::AddTransactionToBalance>::new();
    
    // Create a resource that refetches when the action completes
    let balance_resource = Resource::new(
        move || add_transaction_to_balance.version().get(),
        |_| async move { get_balance().await }
    );

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

        <div>
            <h2>"Current Balance"</h2>
            <Suspense fallback=move || view! { <p>"Loading balance..."</p> }>
                {move || {
                    balance_resource.get().map(|result| {
                        match result {
                            Ok(balance) => view! {
                                <div style="padding: 10px; background-color: #1a3448; border: 1px solid #4682b4; margin-bottom: 20px;">
                                    <p style="font-size: 1.2em; font-weight: bold;">"Balance: €" {balance}</p>
                                </div>
                            }.into_any(),
                            Err(err) => view! {
                                <div style="padding: 10px; background-color: #6e373f; border: 1px solid #f44336; margin-bottom: 20px;">
                                    <p style="color: red;">"Error loading balance: " {err.to_string()}</p>
                                </div>
                            }.into_any(),
                        }
                    })
                }}
            </Suspense>
        </div>

        <div>
            <h2>"Add a transaction"</h2>
            <ActionForm action=add_transaction_to_balance>
                <label for="number">"Amount deposited: € "</label>
                <input type="number" name="number" id="number" step="0.01" required />
                <input type="submit" value="Add" />
            </ActionForm>

            // Display the result of the action
            {move || {
                add_transaction_to_balance.value().get().map(|result| {
                    match result {
                        Ok(new_balance) => {
                            // Get current date/time using utility function
                            let time_str = get_formatted_now();
                            
                            view! {
                                <div style="margin-top: 10px; padding: 10px; background-color: #4b7f4b; border: 1px solid #4caf50;">
                                    <p>"Transaction added at " {time_str} ". New balance: €" {new_balance}</p>
                                </div>
                            }.into_any()
                        },
                        Err(err) => view! {
                            <div style="margin-top: 10px; padding: 10px; background-color: #6e373f; border: 1px solid #f44336;">
                                <p style="color: red;">"Error: " {err.to_string()}</p>
                            </div>
                        }.into_any(),
                    }
                })
            }}
        </div>

        // <a href="/">"Back to Home"</a>
    }
}
