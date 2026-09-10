impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut ret = vec![1; nums.len()];
        let mut l = 0;
        let mut r = nums.len()-1;

        let mut lv = 1;
        let mut rv = 1;
        loop {
            ret[l] = ret[l] * lv;
            ret[r] = ret[r] * rv;

            rv = rv * nums[r];
            lv = lv * nums[l];

            if r == 0 {
                break
            }
            l += 1;
            r -= 1;
        }
        return ret;
    }
}
