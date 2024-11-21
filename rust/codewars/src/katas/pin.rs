use regex::Regex;

pub fn validate_pin(pin: &str) -> bool {
    Regex::new(r"^(\d{4}|\d{6})$").unwrap().is_match(pin)
}
