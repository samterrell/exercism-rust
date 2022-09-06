use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let mut count = 0;
    if !"ACGT".contains(nucleotide) {
        return Err(nucleotide);
    }
    for c in dna.chars() {
        match c {
            _ if c == nucleotide => count += 1,
            'A' | 'C' | 'G' | 'T' => (),
            _ => return Err(c),
        }
    }
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut res = HashMap::new();
    res.insert('A', 0);
    res.insert('C', 0);
    res.insert('G', 0);
    res.insert('T', 0);
    for c in dna.chars() {
        if let Some(v) = res.get_mut(&c) {
            *v += 1;
        } else {
            return Err(c);
        }
    }
    Ok(res)
}
