struct Solution;

impl Solution {
    pub fn binary_prefix_divisible_by_5(nums: Vec<i32>) -> Vec<bool> {
        let mut result = Vec::with_capacity(nums.len());
        let mut x = 0;
        for num in nums.iter() {
            x = (x << 1 | num) % 5;
            result.push(x == 0);
        }
        result
    }
}

#[test]
fn test1() {
    let nums = vec![0,1,1];
    assert_eq!(Solution::binary_prefix_divisible_by_5(nums), vec![true,false,false]);
}

#[test]
fn test2() {
    let nums = vec![1,1,1];
    assert_eq!(Solution::binary_prefix_divisible_by_5(nums), vec![false,false,false]);
}