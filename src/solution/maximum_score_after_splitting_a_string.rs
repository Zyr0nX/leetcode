use std::cmp;

struct Solution;

impl Solution {
    pub fn maximum_score_after_splitting_a_string(s: String) -> i32 {
        let mut ones = s.chars().filter(|&c| c == '1').count();
        let mut zeros = 0;
        let mut score = 0;
        for c in s[..s.len() - 1].chars() {
            if c == '1' {
                ones -= 1;
            } else {
                zeros += 1;
            }

            score = cmp::max(score, ones + zeros);
        }
        score as i32
        
    }
}

#[test]
fn test1() {
    let s = String::from("011101");
    assert_eq!(Solution::maximum_score_after_splitting_a_string(s), 5);
}

#[test]
fn test2() {
    let s = String::from("00111");
    assert_eq!(Solution::maximum_score_after_splitting_a_string(s), 5);
}

#[test]
fn test3() {
    let s = String::from("1111");
    assert_eq!(Solution::maximum_score_after_splitting_a_string(s), 3);
}

#[test]
fn test4() {
    let s = String::from("00");
    assert_eq!(Solution::maximum_score_after_splitting_a_string(s), 1);
}