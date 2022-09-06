pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec![String::from(""); digits.len() + 1];
    }
    digits
        .as_bytes()
        .windows(len)
        .map(|bytes| String::from_utf8_lossy(bytes).into_owned())
        .collect()
}
