struct Solution;

impl Solution {
    pub fn maximum_ascending_subarrays_sum(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut local_max = 0;
        for (i, &num) in nums.iter().enumerate() {
            if i == 0 || nums[i - 1] < num {
                local_max += num;
            }
            else {
                local_max = num;
            }
            if local_max > max {
                max = local_max;
            }
        }
        max
    }
}

#[test]
fn test1() {
    let nums = vec![10,20,30,5,10,50];
    assert_eq!(Solution::maximum_ascending_subarrays_sum(nums), 65);
}

#[test]
fn test2() {
    let nums = vec![10,20,30,40,50] ;
    assert_eq!(Solution::maximum_ascending_subarrays_sum(nums), 150);
}

#[test]
fn test3() {
    let nums = vec![3,6,10,1,8,9,9,8,9];
    assert_eq!(Solution::maximum_ascending_subarrays_sum(nums), 19);
}