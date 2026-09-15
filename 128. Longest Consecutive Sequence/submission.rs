use std::collections::HashSet;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let num_set: HashSet<i32> = nums.into_iter().collect();
        let mut ans = 0;

        for &num in &num_set{
            if !num_set.contains(&(num-1)){
                let mut current_num = num;
                let mut current_streak = 1;

                while num_set.contains(&(current_num + 1)){
                    current_num += 1;
                    current_streak += 1;
                }
                ans = ans.max(current_streak);
            }
        } 
        ans
    }
}
