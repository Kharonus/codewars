pub fn stringy(size: usize) -> String {
    // Clever solution with cycle, take and collect
    // "10".chars().cycle().take(size).collect()

    let array = vec![0; size];

    array.iter().enumerate().fold(
        String::new(),
        |acc, (idx, _)| {
            acc + if idx % 2 == 0 { "1" } else { "0" }
        }
    )
}