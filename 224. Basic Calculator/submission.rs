impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut ans = 0;
        let mut num = 0;
        let mut sign = 1;
        let mut stack = vec![sign];

        for c in s.chars(){
            if c.is_digit(10){
                num = num * 10 + c.to_digit(10).unwrap() as i32;
            }else if c == '('{
                stack.push(sign)
            }else if c == ')'{
                stack.pop();
            }else if c == '+' || c == '-'{
                ans += sign * num;
                sign = (if c == '+' {1}else {-1}) * stack.last().unwrap();
                num = 0;
            }
        }
        ans + sign * num
    }
}
