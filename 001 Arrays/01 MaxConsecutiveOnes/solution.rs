use std::cmp::max;

struct Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut max_consecutive_count = 0;
        let mut consecutive_count = 0;
        for num in nums {
            if num == 1 {
                consecutive_count += 1;
            } else {
                max_consecutive_count = max(consecutive_count, max_consecutive_count);
                consecutive_count = 0;
            }
        }
        return max(max_consecutive_count, consecutive_count);
    }
}

// https://leetcode.com/explore/learn/card/fun-with-arrays/521/introduction/3238/
fn main() {
    // Test 1
    assert_eq!(
        Solution::find_max_consecutive_ones(vec![1,1,0,1,1,1]),
        3);

    // Test 2
    assert_eq!(
        Solution::find_max_consecutive_ones(vec![1,0,1,1,0,1]),
        2);

    println!("Success!");
}
