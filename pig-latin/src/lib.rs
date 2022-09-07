macro_rules! some_vowel {
    () => {
        Some('a')
            | Some('e')
            | Some('i')
            | Some('o')
            | Some('u')
            | Some('A')
            | Some('E')
            | Some('I')
            | Some('O')
            | Some('U')
    };
}

pub fn translate(input: &str) -> String {
    input
        .split(' ')
        .map(translate_word)
        .fold(String::from(""), |l, r| {
            if l == "" {
                l + &r
            } else {
                l + " " + &r
            }
        })
}

fn translate_word(input: &str) -> String {
    let mut offset = 0;
    let mut iter = input.chars();
    loop {
        match iter.next() {
            Some('x') | Some('X') if offset == 0 => match iter.next() {
                Some('r') | Some('R') | None => return split_pig(input, 0),
                some_vowel!() => return split_pig(input, 1),
                _ => offset += 2,
            },
            Some('y') | Some('Y') if offset == 0 => match iter.next() {
                Some('t') | Some('T') | None => return split_pig(input, 0),
                some_vowel!() => return split_pig(input, 1),
                _ => offset += 2,
            },
            Some('q') | Some('Q') => match iter.next() {
                Some('u') | Some('U') => return split_pig(input, offset + 2),
                Some('a') | Some('e') | Some('i') | Some('o') | Some('A') | Some('E')
                | Some('I') | Some('O') => return split_pig(input, offset + 1),
                _ => offset += 2,
            },
            some_vowel!() | Some('y') | Some('Y') => return split_pig(input, offset),
            None => return split_pig(input, 0),
            _ => offset += 1,
        }
    }
}

fn split_pig(input: &str, at: usize) -> String {
    String::from(&input[at..]) + &input[0..at] + &"ay"
}
