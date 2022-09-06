/// Determine whether a sentence is a pangram.

pub fn is_pangram(sentence: &str) -> bool {
    let mut bits = 0u32;
    for b in sentence.bytes() {
        match b {
            b'a'..=b'z' => bits |= 1 << b - b'a',
            b'A'..=b'Z' => bits |= 1 << b - b'A',
            _ => (),
        }
    }
    bits == (1 << 26) - 1
}
