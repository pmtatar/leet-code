use std::cmp::max;

struct Solution;

impl Solution {
    pub fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {
        let end_index = arr.len() - 1;
        for index in (0..end_index).rev() {
            let curr_num = arr[index];
            arr[index] = arr[end_index];
            arr[end_index] = max(arr[index], curr_num);
        }
        arr[end_index] = -1;
        return arr;
    }
}

// https://leetcode.com/explore/learn/card/fun-with-arrays/527/searching-for-items-in-an-array/3250/
fn main() {
    // Test 1
    let mut arr1 = vec![17,18,5,4,6,1];
    assert_eq!(
        Solution::replace_elements(arr1),
        vec![18,6,6,6,1,-1]);

    // Test 2
    let mut arr2 = vec![400];
    assert_eq!(
        Solution::replace_elements(arr2),
        vec![-1]);

    println!("Success!");
}
