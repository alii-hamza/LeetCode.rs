impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut seen_numbers = std::collections::HashSet::new();
        for num in nums{
            if seen_numbers.contains(&num){
                return true;
            }
            seen_numbers.insert(num);
        }
        return false;
    }
}
