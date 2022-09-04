use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let cap_word = word.to_uppercase();
    let chars = canonical(&cap_word);
    let mut res: HashSet<&'a str> = HashSet::new();
    for candidate in possible_anagrams {
        if chars == canonical(candidate) && *candidate.to_uppercase() != cap_word {
            res.insert(candidate);
        }
    }
    res
}

fn canonical(word: &str) -> Vec<char> {
    let mut chars: Vec<char> = word.to_uppercase().chars().collect();
    chars.sort_unstable();
    chars
}
