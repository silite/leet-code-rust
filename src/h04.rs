#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Solution;
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut res = vec![];
        let left_end = nums1.len();
        let right_end = nums2.len();
        let mut end = (left_end + right_end) / 2 + 1;
        let mut left = 0;
        let mut right = 0;
        while left < left_end && right < right_end && end > 0 {
            let left_item = nums1[left];
            let right_item = nums2[right];
            if left_item < right_item {
                res.push(left_item);
                left += 1;
                end -= 1;
            } else if left_item > right_item {
                res.push(right_item);
                right += 1;
                end -= 1;
            } else {
                res.push(left_item);
                left += 1;
                end -= 1;
                if end > 0 {
                    res.push(right_item);
                    right += 1;
                    end -= 1;
                }
            }
        }
        if end != 0 {
            let (target, mut index) = if left == left_end {
                (&nums2, right)
            } else {
                (&nums1, left)
            };
            while end > 0 {
                end -= 1;
                res.push(target[index]);
                index += 1;
            }
        }
        if (left_end + right_end) % 2 == 1 {
            return *res.last().unwrap() as f64;
        } else {
            let len = res.len();
            return (res[len - 1] as f64 + res[len - 2] as f64) / 2.0;
        }
    }
}

#[test]
fn test() {
    let res = Solution::find_median_sorted_arrays([2, 2, 4, 4].to_vec(), [2, 2, 4, 4].to_vec());
    println!("{}", res);
}
