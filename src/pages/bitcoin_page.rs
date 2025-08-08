use crate::components::Header;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rate {
    price: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RatesResponse {
    products: Vec<Rate>,
}

#[server]
pub async fn get_rates() -> Result<RatesResponse, ServerFnError> {
    let response = reqwest::get("https://dummyjson.com/products?limit=2&select=price")
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let rates: RatesResponse = response
        .json()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(rates)
}

/// Renders the Bitcoin page
#[component]
pub fn BitcoinPage() -> impl IntoView {
    let rates_resource = Resource::new(
        || (),
        |_| async move {
            get_rates().await
        },
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
                                    <div>
                                        <h3>"Rates:"</h3>
                                        {rates_response.products.into_iter().enumerate().map(|(i, rate)| {
                                            view! {
                                                <p>"Rate " {i + 1} ": €" {rate.price}</p>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                }.into_any()
                            }
                            Err(err) => {
                                let error_msg = err.to_string();
                                view! {
                                    <div>
                                        <h3>"Error:"</h3>
                                        {vec![view! { <p style="color: red;">"Error loading rates: " {error_msg}</p> }]}
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
