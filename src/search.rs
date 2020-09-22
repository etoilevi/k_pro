use std::{collections::HashMap, hash::Hash};

pub fn count_freq<I, T>(iter: I) -> HashMap<T, usize>
where
    I: Iterator<Item = T>,
    T: Hash + Eq,
{
    let mut result = HashMap::new();
    for a in iter {
        let entry = result.entry(a).or_default();
        *entry += 1;
    }
    result
}

#[test]
fn test_count_freq() {
    let freq = count_freq("aabbbc".chars());
    assert_eq!(freq.get(&'a'), Some(&2));
    assert_eq!(freq.get(&'b'), Some(&3));
    assert_eq!(freq.get(&'c'), Some(&1));
    assert_eq!(freq.get(&'d'), None);
}
