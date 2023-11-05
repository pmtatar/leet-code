struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut trail_index = 0;
        for index in 0..nums.len() {
            if nums[index] == 0 {
                continue;
            }
            nums[trail_index] = nums[index];
            trail_index += 1;
        }
        while trail_index < nums.len() {
            nums[trail_index] = 0;
            trail_index += 1;
        }
    }
}

// https://leetcode.com/explore/learn/card/fun-with-arrays/511/in-place-operations/3157/
fn main() {
    // Test 1
    let mut arr1 = vec![0,1,0,3,12];
    Solution::move_zeroes(&mut arr1);
    assert_eq!(arr1, vec![1,3,12,0,0]);

    // Test 2
    let mut arr2 = vec![0];
    Solution::move_zeroes(&mut arr2);
    assert_eq!(arr2, vec![0]);

    println!("Success!");
}
