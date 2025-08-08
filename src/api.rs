use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rate {
    pub price: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RatesResponse {
    pub products: Vec<Rate>,
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
