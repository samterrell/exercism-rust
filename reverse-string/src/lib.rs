pub fn reverse(input: &str) -> String {
    input.chars().rfold(String::new(), |mut acc, c| {
        acc.push(c);
        acc
    })
}
