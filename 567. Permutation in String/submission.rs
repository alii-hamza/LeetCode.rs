impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let n1 = s1.len();
        let n2 = s2.len();
        
        // If s1 is longer than s2, s2 cannot contain a permutation of s1
        if n1 > n2 {
            return false;
        }

        // Convert strings to byte slices for O(1) indexing performance
        let s1_bytes = s1.as_bytes();
        let s2_bytes = s2.as_bytes();

        // Frequency arrays for lowercase letters 'a' through 'z'
        let mut s1_counts = [0; 26];
        let mut s2_counts = [0; 26];

        // Initialize the count arrays for s1 and the first window of s2
        for i in 0..n1 {
            s1_counts[(s1_bytes[i] - b'a') as usize] += 1;
            s2_counts[(s2_bytes[i] - b'a') as usize] += 1;
        }

        // If the initial window matches, we found a permutation immediately
        if s1_counts == s2_counts {
            return true;
        }

        // Slide the window across s2
        for r in n1..n2 {
            // Add the new character entering the window from the right
            s2_counts[(s2_bytes[r] - b'a') as usize] += 1;
            
            // Remove the old character leaving the window from the left
            let l = r - n1;
            s2_counts[(s2_bytes[l] - b'a') as usize] -= 1;

            // Direct array comparison in Rust is highly optimized and checks all elements
            if s1_counts == s2_counts {
                return true;
            }
        }
        false
    }
}
