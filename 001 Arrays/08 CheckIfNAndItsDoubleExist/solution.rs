struct Solution;

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        for index1 in 0..(arr.len() - 1) {
            for index2 in (index1+1)..arr.len() {
                if arr[index1] == 2 * arr[index2] || arr[index2] == 2 * arr[index1] {
                    return true;
                }
            }
        }
        return false;
    }
}

// https://leetcode.com/explore/learn/card/fun-with-arrays/527/searching-for-items-in-an-array/3250/
fn main() {
    // Test 1
    let mut arr1 = vec![10,2,5,3];
    assert_eq!(
        Solution::check_if_exist(arr1),
        true);

    // Test 2
    let mut arr2 = vec![3,1,7,11];
    assert_eq!(
        Solution::check_if_exist(arr2),
        false);

    println!("Success!");
}
