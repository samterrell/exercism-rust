pub fn nth(n: usize) -> u32 {
    let mut primes: Vec<u32> = Vec::<u32>::with_capacity(n);
    let mut candidate = 2;
    while primes.len() <= n {
        if is_prime(&primes, candidate) {
            primes.push(candidate);
        }
        candidate += 1;
    }
    primes.pop().unwrap()
}

fn is_prime(primes: &[u32], candidate: u32) -> bool {
    for p in primes {
        if p * p > candidate {
            return true;
        }
        if candidate % p == 0 {
            return false;
        }
    }
    true
}
