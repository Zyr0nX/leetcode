struct Solution;

impl Solution {
    pub fn merge_intervals(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
        let mut merged: Vec<Vec<i32>> = Vec::new();
        for interval in intervals.iter() {
            if merged.is_empty() || merged.last().unwrap()[1] < interval[0] {
                merged.push(interval.to_vec());
            } else {
                merged.last_mut().unwrap()[1] = merged[merged.len() - 1][1].max(interval[1]);
            }
        }
        return merged;
    }
}

#[test]
fn test1() {
    let intervals: Vec<Vec<i32>> = vec![vec![1,3],vec![2,6],vec![8,10],vec![15,18]];
    assert_eq!(Solution::merge_intervals(intervals), vec![vec![1,6],vec![8,10],vec![15,18]]);
}

// #[test]
// fn test2() {
//     let nums = vec![1,1,1];
//     assert_eq!(Solution::binary_prefix_divisible_by_5(nums), vec![false,false,false]);
// }