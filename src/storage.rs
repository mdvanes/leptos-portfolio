use std::sync::{Mutex, LazyLock};
use std::collections::HashMap;
use crate::components::CurrencySymbol;

// In-memory balance storage - now a HashMap from CurrencySymbol to f64
pub static BALANCES: LazyLock<Mutex<HashMap<CurrencySymbol, f64>>> = LazyLock::new(|| Mutex::new(HashMap::new()));
