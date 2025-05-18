use indexmap::IndexMap;

/// Sums all values in an IndexMap where keys and values are both usize
pub fn example_fn(map: IndexMap<usize, usize>) -> usize {
    map.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;

    #[test]
    fn test_example_fn() {
        let mut map = IndexMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        map.insert(3, 30);
        
        assert_eq!(example_fn(map), 60);
    }
}