use std::collections::HashMap;

pub fn word_frequencies(input: &str) -> HashMap<String, usize> {
    input
        .split_whitespace()
        .fold(HashMap::new(), |mut map, word| {
            *map.entry(word.to_string()).or_insert(0) += 1;
            map
        })
}
