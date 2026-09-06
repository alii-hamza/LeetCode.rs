use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for word in strs{
            let mut chars: Vec<char> = word.chars().collect();
            chars.sort();
            let sorted_chars = chars.into_iter().collect();
            map.entry(sorted_chars).or_default().push(word);
        }
        map.into_values().collect()
    }
}
