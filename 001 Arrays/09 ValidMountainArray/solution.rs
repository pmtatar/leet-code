struct Solution;

impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        if arr.len() < 3 {
            return false;
        }
        let mut index = 1;
        while index < arr.len() && arr[index - 1] < arr[index] {
            index += 1;
        }
        if index == 1 || index == arr.len() || arr[index - 1] == arr[index] {
            return false;
        }
        while index < arr.len() && arr[index - 1] > arr[index] {
            index += 1;
        }
        return index == arr.len();
    }
}

// https://leetcode.com/explore/learn/card/fun-with-arrays/527/searching-for-items-in-an-array/3250/
fn main() {
    // Test 1
    let mut arr1 = vec![3,5,5];
    assert_eq!(
        Solution::valid_mountain_array(arr1),
        false);

    // Test 2
    let mut arr2 = vec![0,3,2,1];
    assert_eq!(
        Solution::valid_mountain_array(arr2),
        true);

    println!("Success!");
}
