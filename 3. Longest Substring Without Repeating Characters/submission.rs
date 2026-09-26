use std::collections::HashSet;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut char_set = HashSet::new();
        let mut s_char: Vec<char> = s.chars().collect();
        let mut l = 0;
        let mut res = 0;

        for r in 0..s_char.len(){
            while char_set.contains(&s_char[r]){
                char_set.remove(&s_char[l]);
                l += 1;
            }
            char_set.insert(s_char[r]);
            res = res.max(r - l + 1);
        }
        res as i32
    }
}
