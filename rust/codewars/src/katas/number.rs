// https://www.codewars.com/kata/5287e858c6b5a9678200083c
// focus on calculating the result without conversion to string
pub fn narcissistic(num: u64) -> bool {
    let sum = digits(num).fold(0, |acc, d| acc + d.pow(num_digits(num)));
    sum == num
}

fn num_digits(n: u64) -> u32 {
    match n {
        0 => 1,
        _ => (n as f64).log10().floor() as u32 + 1
    }
}

fn digits(n: u64) -> impl Iterator<Item=u64> {
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