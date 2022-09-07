pub fn encode(source: &str) -> String {
    let mut res = String::new();
    let mut last = None;
    let mut count = 0;
    for c in source.chars() {
        if Some(c) == last {
            count += 1;
            continue;
        }
        append(&mut res, count, last);
        last = Some(c);
        count = 1;
    }
    append(&mut res, count, last);
    res
}

pub fn decode(source: &str) -> String {
    let mut buffer = String::new();
    let mut count = 0;
    for c in source.chars() {
        if let Some(i) = c.to_digit(10) {
            count *= 10;
            count += i;
        } else {
            if count == 0 {
                buffer.push(c);
            } else {
                while count > 0 {
                    buffer.push(c);
                    count -= 1;
                }
            }
        }
    }
    buffer
}

fn append(str: &mut String, count: usize, c: Option<char>) {
    match c {
        Some(c) if count == 1 => str.push(c),
        Some(c) if count > 1 => {
            str.push_str(&count.to_string());
            str.push(c)
        }
        _ => (),
    }
}
