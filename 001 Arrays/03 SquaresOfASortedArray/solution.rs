struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut zero_index = 0;
        while zero_index < nums.len() && nums[zero_index] < 0 {
            zero_index += 1;
        }
        let mut neg_index = zero_index.wrapping_sub(1);
        let mut pos_index = zero_index;
        let mut result = vec![0; nums.len()];
        let mut index = 0;
        while neg_index != usize::MAX && pos_index < nums.len() {
            let neg_sq = nums[neg_index] * nums[neg_index];
            let pos_sq = nums[pos_index] * nums[pos_index];
            if neg_sq < pos_sq {
                result[index] = neg_sq;
                neg_index = neg_index.wrapping_sub(1);
            } else {
                result[index] = pos_sq;
                pos_index += 1;
            }
            index += 1;
        }
        while neg_index != usize::MAX {
            result[index] = nums[neg_index] * nums[neg_index];
            neg_index = neg_index.wrapping_sub(1);
            index += 1;
        }
        while pos_index < nums.len() {
            result[index] = nums[pos_index] * nums[pos_index];
            pos_index += 1;
            index += 1;
        }
        return result;
    }
}

// https://leetcode.com/explore/learn/card/fun-with-arrays/521/introduction/3240/
fn main() {
    // Test 1
    assert_eq!(
        Solution::sorted_squares(vec![-4,-1,0,3,10]),
        vec![0,1,9,16,100]);

    // Test 2
    assert_eq!(
        Solution::sorted_squares(vec![-7,-3,2,3,11]),
        vec![4,9,9,49,121]);

    // Test 3
    assert_eq!(
        Solution::sorted_squares(vec![1, 2, 3, 4, 5 ,6]),
        vec![1, 4, 9, 16, 25, 36]);

    println!("Success!");
}
