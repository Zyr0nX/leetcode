struct Solution;

impl Solution {
    pub fn minimum_one_bit_operations_to_make_integers_zero(n: i32) -> i32 {
        let mut k: i32 = 0;
        while n > 2i32.pow(k as u32) {
            k += 1;
        }
        k -= 1;
        2i32.pow(k as u32 +1) - 1 - Self::minimum_one_bit_operations_to_make_integers_zero(2i32.pow(k as u32) ^ n)
    }
}

#[test]
fn test1() {
    let nums = vec![0,1,1];
    assert_eq!(Solution::minimum_one_bit_operations_to_make_integers_zero(3), 2);
}

#[test]
fn test2() {
    let nums = vec![1,1,1];
    assert_eq!(Solution::minimum_one_bit_operations_to_make_integers_zero(6), 4);
}