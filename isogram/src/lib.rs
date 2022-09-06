pub fn check(candidate: &str) -> bool {
    let mut seen = [false; 26];
    for b in candidate.bytes() {
        let offset = match b {
            b'a'..=b'z' => b - b'a',
            b'A'..=b'Z' => b - b'A',
            _ => continue,
        };
        let seen = seen.get_mut(offset as usize).unwrap();
        if *seen {
            return false;
        } else {
            *seen = true;
        }
    }
    true
}
