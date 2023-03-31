#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = arr.len() - 1;
        while left <= right {
            if left < right && arr[left] <= arr[left + 1] {
                left += 1;
            } else if left < right && arr[right - 1] <= arr[right] {
                right -= 1;
            } else {
                break;
            }
        }
        right as i32 - left as i32
    }
}

#[test]
fn test() {
    let res = Solution::find_length_of_shortest_subarray(vec![1, 2, 3, 10, 4, 2, 3, 5]);
    println!("{}", res);
}
