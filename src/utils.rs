use chrono::prelude::*;

/// Gets the current local date and time formatted as "time date"
pub fn get_formatted_now() -> String {
    let now = Local::now();
    now.format("%H:%M %d-%m-%Y").to_string()
}
