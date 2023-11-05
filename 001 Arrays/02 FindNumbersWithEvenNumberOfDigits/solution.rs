struct Solution;

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut nums_with_even_digits = 0;
        for num in nums {
            let mut digit_count = 1;
            let mut quotient = num / 10;
            while quotient > 0 {
                digit_count += 1;
                quotient /= 10;
            }
            if digit_count % 2 == 0 {
                nums_with_even_digits += 1;
            }
        }
        return nums_with_even_digits;
    }
}

// https://leetcode.com/explore/learn/card/fun-with-arrays/521/introduction/3237/
fn main() {
    // Test 1
    assert_eq!(
        Solution::find_numbers(vec![12,345,2,6,7896]),
        2);

    // Test 2
    assert_eq!(
        Solution::find_numbers(vec![555,901,482,1771]),
        1);

    println!("Success!");
}
