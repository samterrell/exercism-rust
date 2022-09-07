use std::cmp::Ordering::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    match num.cmp(&(1..num).filter(|f| num % f == 0).sum()) {
        Less => Some(Classification::Abundant),
        Equal => Some(Classification::Perfect),
        Greater => Some(Classification::Deficient),
    }
}
