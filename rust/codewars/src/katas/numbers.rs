// https://www.codewars.com/kata/5287e858c6b5a9678200083c
// focus on calculating the result without conversion to string
pub fn narcissistic(num: u64) -> bool {
    let sum = digits(num).fold(0, |acc, d| acc + d.pow(num_digits(num)));
    sum == num
}

fn num_digits(n: u64) -> u32 {
    match n {
        0 => 1,
        _ => (n as f64).log10().floor() as u32 + 1,
    }
}

fn digits(n: u64) -> impl Iterator<Item = u64> {
    let mut num = n;
    std::iter::from_fn(move || {
        if num == 0 {
            None
        } else {
            let digit = num % 10;
            num /= 10;
            Some(digit)
        }
    })
}

// https://www.codewars.com/kata/554b4ac871d6813a03000035
pub fn high_and_low(numbers: &str) -> String {
    let tuple = numbers
        .split_whitespace()
        .fold((i32::MIN, i32::MAX), |(max, min), n| {
            let num = n.parse::<i32>().unwrap();
            (max.max(num), min.min(num))
        });

    format!("{} {}", tuple.0, tuple.1)
}
