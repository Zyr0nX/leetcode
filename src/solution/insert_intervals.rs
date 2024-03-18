struct Solution;

impl Solution {
    pub fn merge_intervals(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return Vec::from([new_interval]);
        } 
        
        let mut left: isize = 0;
        let mut right: isize =  intervals.len() as isize - 1;
        let target = new_interval[0];

        while left <= right {
            let mid = (left + right) / 2;

            if intervals[mid as usize][0] < target {
                left = mid + 1;
            }
            else {
                right = mid - 1;
            }
        }

        intervals.insert(left.try_into().unwrap(), new_interval);

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
    let intervals: Vec<Vec<i32>> = vec![vec![1,3],vec![6,9]];
    assert_eq!(Solution::merge_intervals(intervals, vec![2,5]), vec![vec![1,5],vec![6,9]]);
}

#[test]
fn test2() {
    let intervals: Vec<Vec<i32>> = vec![vec![1,5]];
    assert_eq!(Solution::merge_intervals(intervals, vec![1,7]), vec![vec![1,7]]);
}