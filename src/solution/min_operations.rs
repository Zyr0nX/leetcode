struct Solution;

use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn min_operations_1(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        for num in nums.iter() {
            if *num < k {
                count += 1;
            }
        }
        count
    }

    pub fn min_operations_2(nums: Vec<i32>, k: i32) -> i32 {
        // let mut heap = BinaryHeap::from(nums);
        // for &num in nums.iter() {
        //     heap.push(Reverse(num as i64));
        // }
        // let mut operations = 0;
        // while let Some(x) = heap.pop() {
        //     if x >= k { 
        //         break;
        //     }
        //     if let Some(y) = heap.pop() {
        //         heap.push(x.min(y) * 2 + x.max(y));
        //     }
        //     operations += 1;
        // }
        // operations
        0
    }
}

#[test]
fn test1() {
    let nums = vec![2,11,10,1,3];
    let k = 10;
    assert_eq!(Solution::min_operations_1(nums, k), 3);
}

#[test]
fn test2() {
    let nums = vec![1,1,2,4,9];
    let k = 1;
    assert_eq!(Solution::min_operations_1(nums, k), 0);
}

#[test]
fn test3() {
    let nums = vec![1,1,2,4,9];
    let k = 9;
    assert_eq!(Solution::min_operations_1(nums, k), 4);
}

#[test]
fn test4() {
    let nums = vec![2,11,10,1,3];
    let k = 10;
    assert_eq!(Solution::min_operations_2(nums, k), 4);
}

#[test]
fn test5() {
    let nums = vec![1,1,2,4,9];
    let k = 20;
    assert_eq!(Solution::min_operations_2(nums, k), 4);
}