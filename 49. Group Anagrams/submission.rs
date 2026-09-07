# ==========================================
# APPROACH 1: Sorting
# ==========================================

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



# ==========================================
# APPROACH 2: Counting (Fast/Optimal)
# ==========================================

use std::collections::HashMap;
impl Solution {
  pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<[u8;26], Vec<String>> = HashMap::with_capacity(strs.len());
    let offset = 'a' as usize;

    for str in strs.into_iter() {
      let mut chars: [u8; 26] = [0; 26];

      for char in str.chars() {
        chars[char.to_ascii_lowercase() as usize - offset] += 1;
      }

      map.entry(chars)
        .and_modify(|v| v.push(str.clone()))
        .or_insert(vec![str]);
    }
    
    map.values().cloned().collect::<Vec<Vec<String>>>()
  }
}
