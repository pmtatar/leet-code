struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut start_index = 0;
        for index in 0..nums.len() {
            if nums[index] == val {
                continue;
            }
            nums[start_index] = nums[index];
            start_index += 1;
        }
        return start_index as i32;
    }
}

// https://leetcode.com/explore/learn/card/fun-with-arrays/525/inserting-items-into-an-array/3253/
fn main() {
    // Test 1
    let mut arr1 = vec![3,2,2,3];
    let k1 = Solution::remove_element(&mut arr1, 3);
    assert_eq!(arr1, vec![2,2,2,3]);
    assert_eq!(k1, 2);

    // Test 2
    let mut arr2 = vec![0,1,2,2,3,0,4,2];
    let k2 = Solution::remove_element(&mut arr2, 2);
    assert_eq!(arr2, vec![0,1,3,0,4,0,4,2]);
    assert_eq!(k2, 5);

    println!("Success!");
}
