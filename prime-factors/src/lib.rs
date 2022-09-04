pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = vec![];
    let mut current = n;
    let mut candidate = 2;
    while candidate * candidate <= current {
        if current % candidate == 0 {
            factors.push(candidate);
            current /= candidate;
        } else {
            candidate += 1
        }
    }
    if current > 1 {
        factors.push(current);
    }
    factors
}
