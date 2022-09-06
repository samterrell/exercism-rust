/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut coef = 10u64;
    let mut check = 0u64;
    for b in isbn.bytes() {
        match b {
            b'0'..=b'9' if coef > 0 => {
                check += (b - b'0') as u64 * coef;
                coef -= 1;
            }
            b'X' | b'x' if coef == 1 => {
                check += 10;
                coef -= 1;
            }
            _ if coef > 0 => (),
            _ => return false,
        }
    }
    coef == 0 && check % 11 == 0
}
