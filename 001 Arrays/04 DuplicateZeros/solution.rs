struct Solution;

impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut extra_size = 0;
        let mut index = 0;
        while index + extra_size < arr.len() {
            if arr[index] == 0 {
                extra_size += 1;
            }
            index += 1;
        }
        index -= 1;
        let mut end_index = arr.len() - 1;
        if index + extra_size == arr.len() {
            arr[end_index] = arr[index];
            index = index.wrapping_sub(1);
            end_index = end_index.wrapping_sub(1);
        }
        while index != usize::MAX {
            if arr[index] == 0 {
                arr[end_index] = 0;
                end_index = end_index.wrapping_sub(1);
            }
            arr[end_index] = arr[index];
            index = index.wrapping_sub(1);
            end_index = end_index.wrapping_sub(1);
        }
    }
}

// https://leetcode.com/explore/learn/card/fun-with-arrays/525/inserting-items-into-an-array/3245/
fn main() {
    // Test 1
    let mut arr1 = vec![1,0,2,3,0,4,5,0];
    Solution::duplicate_zeros(&mut arr1);
    assert_eq!(arr1, [1,0,0,2,3,0,0,4]);

    // Test 2
    let mut arr2 = vec![1,2,3];
    Solution::duplicate_zeros(&mut arr2);
    assert_eq!(arr2, [1,2,3]);

    // Test 3
    let mut arr3 = vec![1];
    Solution::duplicate_zeros(&mut arr3);
    assert_eq!(arr3, [1]);

    // Test 4
    let mut arr4 = vec![1];
    Solution::duplicate_zeros(&mut arr4);
    assert_eq!(arr4, [1]);

    // Test 5
    let mut arr5 = vec![0,0,0,0,0,0,0];
    Solution::duplicate_zeros(&mut arr5);
    assert_eq!(arr5, [0,0,0,0,0,0,0]);

    println!("Success!");
}
