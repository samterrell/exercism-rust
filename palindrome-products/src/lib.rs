/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        // 58.6s
        // let v = value.to_string();
        // if v.chars().rev().zip(v.chars()).all(|(l, r)| l == r) {
        //     Some(Palindrome(value))
        // } else {
        //     None
        // }

        // 10s
        // let mut this = value;
        // let mut that = 0;
        // while this > 0 {
        //     that *= 10;
        //     that += this % 10;
        //     this /= 10;
        // }
        // if value == that {
        //     Some(Palindrome(value))
        // } else {
        //     None
        // }

        // 7.3s
        let mut this = value;
        let mut other = 0;
        while this > other {
            let rem = this % 10;
            this /= 10;
            if rem == 0 && other == 0 {
                // shortcut out of trailing 0's
                return None;
            }
            if this == other {
                // Catches odd digit palindromes
                return Some(Palindrome(value));
            }
            other = other * 10 + rem;
        }
        if this == other {
            // Even digit palindromes
            Some(Palindrome(value))
        } else {
            None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let res = (min..=max)
        .flat_map(|l| (l..=max).map(move |r| l * r))
        .flat_map(|v| Palindrome::new(v))
        .fold(None, |res, p| match res {
            Some((min, max)) if p < min => Some((p, max)),
            Some((min, max)) if p > max => Some((min, p)),
            None => Some((p, p)),
            res => res,
        });
    res
}
