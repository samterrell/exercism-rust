pub fn abbreviate(phrase: &str) -> String {
    let mut last: Option<char> = None;
    phrase
        .chars()
        .filter(|v| {
            let res = match (last, v) {
                (_, ' ' | '-' | '_' | '\t') => false,
                (None | Some(' ') | Some('-') | Some('_') | Some('\t'), _) => true,
                (Some(last), v) if v.is_uppercase() && last.is_lowercase() => true,
                _ => false,
            };
            last = Some(*v);
            res
        })
        .flat_map(|c| c.to_uppercase())
        .collect()
}
