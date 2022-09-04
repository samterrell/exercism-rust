pub fn reply(message: &str) -> &str {
    let message = message.trim_end();
    match message.as_bytes().last() {
        None => "Fine. Be that way!",
        Some(b'?') if is_shouting(message) => "Calm down, I know what I'm doing!",
        Some(b'?') => "Sure.",
        _ if is_shouting(message) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}

fn is_shouting(message: &str) -> bool {
    let mut any_upper = false;
    for c in message.chars() {
        if c.is_lowercase() {
            return false;
        }
        if c.is_uppercase() {
            any_upper = true;
        }
    }
    any_upper
}
