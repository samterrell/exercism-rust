use std::cmp::Ordering::*;
pub fn find<U: AsRef<[T]>, T: Ord>(array: U, key: T) -> Option<usize> {
    let array = array.as_ref();
    let mid = array.len() / 2;
    match array.get(mid)?.cmp(&key) {
        Equal => Some(mid),
        Less => Some(mid + 1 + find(&array.as_ref()[mid + 1..], key)?),
        Greater => find(&array.as_ref()[..mid], key),
    }
}
