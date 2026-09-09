impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freq = std::collections::HashMap::new();
        for &x in nums.iter() {
            *freq.entry(x).or_insert(0) += 1;
        }
        let mut sorted = vec![Vec::new(); nums.len()];
        for (&x, &count) in freq.iter() {
            sorted[nums.len() - count].push(x);
        }
        sorted.into_iter().flatten().take(k as usize).collect()
    }
}
