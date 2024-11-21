// https://www.codewars.com/kata/55f8a9c06c018a0d6e000132
use regex::Regex;

pub fn validate_pin(pin: &str) -> bool {
    Regex::new(r"^(\d{4}|\d{6})$").unwrap().is_match(pin)
}
