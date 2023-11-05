struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut index1 = (m - 1) as usize;
        let mut index2 = (n - 1) as usize;
        let mut end_index = nums1.len() - 1;
        while index1 != usize::MAX && index2 != usize::MAX {
            if nums1[index1] > nums2[index2] {
                nums1[end_index] = nums1[index1];
                index1 = index1.wrapping_sub(1);
            } else {
                nums1[end_index] = nums2[index2];
                index2 = index2.wrapping_sub(1);
            }
            end_index = end_index.wrapping_sub(1);
        }
        while index1 != usize::MAX {
            nums1[end_index] = nums1[index1];
            index1 = index1.wrapping_sub(1);
            end_index = end_index.wrapping_sub(1);
        }
        while index2 != usize::MAX {
            nums1[end_index] = nums2[index2];
            index2 = index2.wrapping_sub(1);
            end_index = end_index.wrapping_sub(1);
        }
    }
}

// https://leetcode.com/explore/learn/card/fun-with-arrays/525/inserting-items-into-an-array/3253/
fn main() {
    // Test 1
    let mut arr1 = vec![1,2,3,0,0,0];
    Solution::merge(&mut arr1, 3, &mut vec![2,5,6], 3);
    assert_eq!(arr1, vec![1,2,2,3,5,6]);

    // Test 2
    let mut arr2 = vec![1];
    Solution::merge(&mut arr2, 1, &mut vec![], 0);
    assert_eq!(arr2, vec![1]);

    // Test 3
    let mut arr3 = vec![0];
    Solution::merge(&mut arr3, 0, &mut vec![1], 1);
    assert_eq!(arr3, vec![1]);

    println!("Success!");
}
