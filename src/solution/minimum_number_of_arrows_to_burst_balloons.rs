struct Solution;

impl Solution {
    pub fn minimum_number_of_arrows_to_burst_balloons(mut points: Vec<Vec<i32>>) -> i32  {
        points.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
        let mut merged: Vec<Vec<i32>> = Vec::new();
        for point in points.iter() {
            if merged.is_empty() || merged.last().unwrap()[1] < point[0] {
                merged.push(point.to_vec());
            } else {
                merged.last_mut().unwrap()[1] = merged[merged.len() - 1][1].min(point[1]);
            }
        }
        return merged.len() as i32;
    }
}

#[test]
fn test1() {
    let points : Vec<Vec<i32>> = vec![vec![10,16],vec![2,8],vec![1,6],vec![7,12]];
    assert_eq!(Solution::minimum_number_of_arrows_to_burst_balloons(points), 2);
}

// #[test]
// fn test2() {
//     let nums = vec![1,1,1];
//     assert_eq!(Solution::binary_prefix_divisible_by_5(nums), vec![false,false,false]);
// }