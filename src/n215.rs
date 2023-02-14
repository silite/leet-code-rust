// quick_sort
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;

use rand::Rng;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums.clone();
        let len = nums.len();
        Solution::quick_sort(&mut nums, 0, len);
        nums[len - k as usize]
    }

    fn quick_sort(arr: &mut Vec<i32>, begin: usize, end: usize) {
        fn partition(arr: &mut Vec<i32>, begin: usize, end: usize) -> usize {
            let mut rng = rand::thread_rng();
            // let sign = rng.gen_range(begin, end + 1);
            let sign = rng.gen_range(begin..end);
            arr.swap(sign, end - 1);
            let (mut i, v) = (begin, arr[end - 1]);
            for j in begin..end - 1 {
                if arr[j] <= v {
                    arr.swap(i, j);
                    i += 1;
                }
            }
            arr.swap(i, end - 1);
            i
        }

        if end - begin < 1 {
            return;
        }
        let mid = partition(arr, begin, end);
        Self::quick_sort(arr, begin, mid);
        Self::quick_sort(arr, mid + 1, end);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        // [3, 1, 5, 2, 6, 4]
        let mut arr = [3, 2, 1, 5, 6, 4].to_vec();
        Solution::quick_sort(&mut arr, 0, 6);

        println!("{:?}", arr);
    }

    #[test]
    fn test_2() {
        let mut arr = [3, 2, 1, 5, 6, 4].to_vec();
        let res = Solution::find_kth_largest(arr, 2);
        println!("{}", res);
    }
}
