use leptos::prelude::*;

// Types for use with https://dummyjson.com/products?delay=500&limit=2&select=price
// use serde::{Deserialize, Serialize};
// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Rate {
//     pub price: f64,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct RatesResponse {
//     pub products: Vec<Rate>,
// }

// Type for use with http://www.randomnumberapi.com/api/v1.0/random?min=55000&max=100000&count=2
pub type RatesResponse = Vec<f64>;

#[server]
pub async fn get_rates() -> Result<RatesResponse, ServerFnError> {
    let response =
        reqwest::get("http://www.randomnumberapi.com/api/v1.0/random?min=55000&max=100000&count=2")
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

    let rates: RatesResponse = response
        .json()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(rates)
}

#[server]
pub async fn add_transaction_to_balance(number: f64) -> Result<f64, ServerFnError> {
    Ok(number)
}
