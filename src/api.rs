use leptos::prelude::*;
use std::sync::{Mutex, LazyLock};
use std::collections::HashMap;
use crate::components::CurrencySymbol;

// In-memory balance storage - now a HashMap from CurrencySymbol to f64
static BALANCES: LazyLock<Mutex<HashMap<CurrencySymbol, f64>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

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
pub async fn add_transaction_to_balance(number: f64, currency: CurrencySymbol) -> Result<f64, ServerFnError> {
    let mut balances = BALANCES.lock().map_err(|e| ServerFnError::new(format!("Failed to lock balances: {}", e)))?;
    let current_balance = balances.get(&currency).unwrap_or(&0.0);
    let new_balance = current_balance + number;
    balances.insert(currency, new_balance);
    Ok(new_balance)
}

#[server]
pub async fn get_balance(currency: CurrencySymbol) -> Result<f64, ServerFnError> {
    let balances = BALANCES.lock().map_err(|e| ServerFnError::new(format!("Failed to lock balances: {}", e)))?;
    let balance = balances.get(&currency).unwrap_or(&0.0);
    Ok(*balance)
}

#[server]
pub async fn reset_balance(currency: Option<CurrencySymbol>) -> Result<(), ServerFnError> {
    let mut balances = BALANCES.lock().map_err(|e| ServerFnError::new(format!("Failed to lock balances: {}", e)))?;
    
    match currency {
        Some(curr) => {
            balances.insert(curr, 0.0);
        }
        None => {
            balances.clear();
        }
    }
    
    Ok(())
}
