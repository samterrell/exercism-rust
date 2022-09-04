/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut even = false;
    let mut acc = 0u8;
    let mut count = 0usize;
    for b in code.bytes().rev() {
        let mut v = match b {
            b' ' => continue,
            x if x >= b'0' && x <= b'9' => x - b'0' as u8,
            _ => return false,
        };
        if even {
            v *= 2;
            if v > 9 {
                v -= 9;
            }
        }
        even = !even;
        acc += v;
        if acc >= 10 {
            acc -= 10;
        }
        count += 1;
    }
    acc == 0 && count > 1
}
