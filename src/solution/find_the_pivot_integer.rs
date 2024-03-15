struct Solution;

impl Solution {
    pub fn find_the_pivot_integer(n: i32) -> i32 {
        let mut right = n;
        let mut left = 1;
        let mut sum_left = left;
        let mut sum_right = right;

        if n == 1 {
            return n
        }

        while left < right {
            if sum_left < sum_right {
                sum_left += left + 1;
                left += 1;
            }
            else if sum_left > sum_right {
                sum_right += right - 1;
                right -= 1;
            }
            else {
                if left + 1 == right - 1 {
                    return left + 1;
                }
                else {
                    sum_left += left + 1;
                    left += 1;
                    sum_right += right - 1;
                    right -= 1;
                }
            }

            if sum_left == sum_right && left + 1 == right - 1 {
                return left + 1;
            }
        }
        return -1;
    }
}

#[test]
fn test1() {
    assert_eq!(Solution::find_the_pivot_integer(8), 6);
}

#[test]
fn test2() {
    assert_eq!(Solution::find_the_pivot_integer(1), 1);
}

#[test]
fn test3() {
    assert_eq!(Solution::find_the_pivot_integer(4), -1);
}
