pub fn fizz_buzz(n: i32) -> Vec<String> {
    let mut res = vec![];
    for i in 0..=n {
        match (i % 3, i % 5) {
            (0, 0) => res.push("fizz buzz".to_string()),
            (0, _) => res.push("fizz".to_string()),
            (_, 0) => res.push("buzz".to_string()),
            _ => (),
        }
    }
    res
}
