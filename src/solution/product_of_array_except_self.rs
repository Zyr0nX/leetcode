struct Solution;

impl Solution {
    pub fn product_of_array_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut output = Vec::with_capacity(nums.len());
        let mut temp = 1; //prefix
        for &num in nums.iter() {
            output.push(temp);
            temp *= num;
        }
        temp = 1; //postfix
        for (i, &num) in nums.iter().enumerate().rev() {
            output[i] *= temp;
            temp *= num;
        }
        output
    }
}

#[test]
fn test1() {
    let nums = vec![1,2,3,4];
    assert_eq!(Solution::product_of_array_except_self(nums), vec![24,12,8,6]);
}

#[test]
fn test2() {
    let nums = vec![-1,1,0,-3,3];
    assert_eq!(Solution::product_of_array_except_self(nums), vec![0,0,9,0,0]);
}