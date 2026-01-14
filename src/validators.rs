use regex::Regex;
use std::collections::HashMap;
use std::any::Any;
use std::time::SystemTime;

//1
pub fn is_email(input: &str) -> bool {
    // Regular expression for basic email validation
    let email_pattern = r"^[\w\.-]+@[\w\.-]+\.\w+$";

    // Create regex object
    let regex = Regex::new(email_pattern).unwrap();

    // Check if input matches the pattern
    if regex.is_match(input) {
        return true;
    }

    false
}

//2
pub fn is_url(input: &str) -> bool {
    let url_pattern = r"^(http|https)://[^\s]+$";
    let regex = Regex::new(url_pattern).unwrap();

    if regex.is_match(input) {
        return true;
    }

    false
}

//3
pub fn is_phone_number(input: &str) -> bool {
    let phone_pattern = r"^\+?\d[\d\s\-]{7,}$";
    let regex = Regex::new(phone_pattern).unwrap();

    regex.is_match(input)
}

//4
pub fn is_strong_password(input: &str) -> bool {

    let password_pattern =
        r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[\W_]).{8,}$";

    let regex = Regex::new(password_pattern).unwrap();

    if regex.is_match(input) {
        true
    } else {
        false
    }
}

//5
pub fn is_empty<T>(value: Option<T>) -> bool {
    // If value is None, it is empty
    if value.is_none() {
        return true;
    }

    false
}

//6
pub fn is_number(value: &dyn Any) -> bool {
    // Check common numeric types
    if value.is::<i32>() || value.is::<i64>() || value.is::<f64>() {
        return true;
    }

    false
}

//7
pub fn is_string(value: &dyn Any) -> bool {
    if value.is::<String>() || value.is::<&str>() {
        return true;
    }

    false
}

//8
pub fn is_array<T>(value: &Vec<T>) -> bool {
    // In Rust, arrays/lists are usually Vec
    if value.len() >= 0 {
        return true;
    }

    false
}

//9
pub fn is_object<K, V>(value: &HashMap<K, V>) -> bool {
    // HashMap is closest to JS object
    if value.len() >= 0 {
        return true;
    }

    false
}

//10
pub fn is_function<F>(_value: &F) -> bool
where
    F: Fn(),
{
    // If it compiles as Fn, it is a function
    true
}

//11
pub fn is_boolean(value: &dyn Any) -> bool {
    if value.is::<bool>() {
        return true;
    }

    false
}

//12
pub fn is_date(value: &dyn Any) -> bool {
    // Using SystemTime as date representation
    if value.is::<SystemTime>() {
        return true;
    }

    false
}

//13
pub fn is_json(input: &str) -> bool {
    let result = serde_json::from_str::<serde_json::Value>(input);

    if result.is_ok() {
        return true;
    }

    false
}

//14
pub fn is_ip_address(input: &str) -> bool {
    if is_ipv4(input) || is_ipv6(input) {
        return true;
    }

    false
}

//15
pub fn is_ipv4(input: &str) -> bool {
    let ipv4_pattern =
        r"^((25[0-5]|2[0-4]\d|[01]?\d\d?)\.){3}(25[0-5]|2[0-4]\d|[01]?\d\d?)$";

    let regex = Regex::new(ipv4_pattern).unwrap();

    regex.is_match(input)
}


//16
pub fn is_ipv6(input: &str) -> bool {
    let ipv6_pattern = r"^([0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}$";
    let regex = Regex::new(ipv6_pattern).unwrap();

    regex.is_match(input)
}

//17
pub fn is_mac_address(input: &str) -> bool {
    let mac_pattern = r"^([0-9A-Fa-f]{2}:){5}[0-9A-Fa-f]{2}$";
    let regex = Regex::new(mac_pattern).unwrap();

    regex.is_match(input)
}

//18
pub fn is_credit_card(input: &str) -> bool {
    let mut digits: Vec<u32> = Vec::new();

    // Extract digits only
    for ch in input.chars() {
        if ch.is_digit(10) {
            digits.push(ch.to_digit(10).unwrap());
        }
    }

    if digits.len() < 13 {
        return false;
    }

    let mut sum = 0;
    let mut should_double = false;

    for i in (0..digits.len()).rev() {
        let mut value = digits[i];

        if should_double {
            value = value * 2;

            if value > 9 {
                value = value - 9;
            }
        }

        sum = sum + value;
        should_double = !should_double;
    }

    sum % 10 == 0
}

//19
pub fn is_visa_card(input: &str) -> bool {
    let visa_pattern = r"^4\d{12}(\d{3})?$";
    let regex = Regex::new(visa_pattern).unwrap();

    if regex.is_match(input) && is_credit_card(input) {
        return true;
    }

    false
}

//20
pub fn is_master_card(input: &str) -> bool {
    let master_pattern = r"^5[1-5]\d{14}$";
    let regex = Regex::new(master_pattern).unwrap();

    if regex.is_match(input) && is_credit_card(input) {
        return true;
    }

    false
}
