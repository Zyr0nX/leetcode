struct Solution;

impl Solution {
    pub fn merge_sorted_array(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut m = m as usize;
        let mut n = n as usize;
        
        while n > 0 {
            if m > 0 && nums1[m - 1] > nums2[n - 1] {
                nums1[m + n - 1] = nums1[m - 1];
                m -= 1;
            } else {
                nums1[m + n - 1] = nums2[n - 1];
                n -= 1;
            }
        }
    }
}

#[test]
fn test1() {
    let mut nums1 = vec![1,2,3,0,0,0];
    Solution::merge_sorted_array(&mut nums1, 3, &mut vec![2,5,6], 3);
    assert_eq!(nums1, vec![1,2,2,3,5,6]);
}

#[test]
fn test2() {
    let mut nums1 = vec![1];
    Solution::merge_sorted_array(&mut nums1, 1, &mut vec![], 0);
    assert_eq!(nums1, vec![1]);
}

#[test]
fn test3() {
    let mut nums1 = vec![0];
    Solution::merge_sorted_array(&mut nums1, 0, &mut vec![1], 1);
    assert_eq!(nums1, vec![1]);
}