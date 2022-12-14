pub fn square(s: u32) -> u64 {
    match s {
        1..=64 => 1 << (s - 1),
        _ => panic!("Square must be between 1 and 64"),
    }
}

// \sum_n=0_64(2^n) = 2^(n+1) - 1 = 0xFFFF_FFFF_FFFF_FFFF = u64::MAX
pub fn total() -> u64 {
    u64::MAX
}
