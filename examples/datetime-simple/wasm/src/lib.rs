mod utils;

use chrono::Datelike;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, datetime-simple!");
}

#[wasm_bindgen]
pub fn add_next_day(input: &str) -> String {
    let mut date = chrono::NaiveDate::parse_from_str(input, "%Y-%m-%d").unwrap();
    date = date + chrono::Duration::days(1);
    date.format("%Y-%m-%d").to_string()
}

#[wasm_bindgen]
pub fn is_weekend(input: &str) -> bool {
    let date = chrono::NaiveDate::parse_from_str(input, "%Y-%m-%d").unwrap();
    date.weekday() == chrono::Weekday::Sat || date.weekday() == chrono::Weekday::Sun
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add_next_day("2020-01-01"), "2020-01-02");
    }
    #[test]
    fn it_works2() {
        assert_eq!(add_next_day("2020-02-27"), "2020-02-28");
        assert_eq!(add_next_day("2020-02-28"), "2020-02-29");
        assert_eq!(add_next_day("2020-02-29"), "2020-03-01");
    }

    #[test]
    fn test_is_weekend() {
        assert_eq!(is_weekend("2023-04-09"), true);
    }
}