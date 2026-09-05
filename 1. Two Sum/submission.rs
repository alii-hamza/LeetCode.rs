impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();

        for (i, &x) in nums.iter().enumerate() {
            if let Some(&j) = map.get(&(target - x)) {
                return vec![j, i as i32];
        }
        map.insert(x, i as i32);
    }
        vec![]
    }
}
