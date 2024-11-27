// https://www.codewars.com/kata/563b74ddd19a3ad462000054
pub fn stringy(size: usize) -> String {
    // Clever solution with cycle, take and collect
    // "10".chars().cycle().take(size).collect();

    let array = vec![0; size];

    array.iter().enumerate().fold(
        String::new(),
        |acc, (idx, _)| {
            acc + if idx % 2 == 0 { "1" } else { "0" }
        },
    )
}

// https://www.codewars.com/kata/51f2d1cafc9c0f745c00037d
pub fn ends_with(word: &str, ending: &str) -> bool {
    word.ends_with(ending)
}

// https://www.codewars.com/kata/5208f99aee097e6552000148
pub fn break_camel_case(s: &str) -> String {
    s.chars().fold(String::new(), |acc, c| {
        let prefix = if c.is_uppercase() { " " } else { "" };
        format!("{}{}{}", acc, prefix, c)
    })
}