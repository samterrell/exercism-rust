// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut map = HashMap::new();
    for word in magazine {
        if let Some(current) = map.get_mut(*word) {
            *current += 1;
        } else {
            map.insert(*word, 1);
        }
    }
    for word in note {
        if let Some(mag) = map.get_mut(*word) {
            if *mag == 0 {
                return false;
            }
            *mag -= 1;
        } else {
            return false;
        }
    }
    true
}
