struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut unique_index = 0;
        for index in 0..nums.len() {
            if nums[unique_index] == nums[index] {
                continue;
            }
            unique_index += 1;
            nums[unique_index] = nums[index];
        }
        return (unique_index + 1) as i32;
    }
}

// https://leetcode.com/explore/learn/card/fun-with-arrays/525/inserting-items-into-an-array/3253/
fn main() {
    // Test 1
    let mut arr1 = vec![1,1,2];
    let k1 = Solution::remove_duplicates(&mut arr1);
    assert_eq!(arr1, vec![1,2,2]);
    assert_eq!(k1, 2);

    // Test 2
    let mut arr2 = vec![0,0,1,1,1,2,2,3,3,4];
    let k2 = Solution::remove_duplicates(&mut arr2);
    assert_eq!(arr2, vec![0,1,2,3,4,2,2,3,3,4]);
    assert_eq!(k2, 5);

    println!("Success!");
}
