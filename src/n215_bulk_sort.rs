#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use rand::Rng;
struct Solution;
impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        fn qss(arr: &mut Vec<i32>, l: usize, r: usize, k: usize) {
            if l >= r {
                return;
            }
            let (le, eq) = partition(arr, l, r);
            if k as i32 <= (le as i32 - 1) {
                qss(arr, l, le - 1, k);
            }
            if k >= eq + 1 {
                qss(arr, eq + 1, r, k);
            }
        }
        fn partition(arr: &mut Vec<i32>, l: usize, r: usize) -> (usize, usize) {
            let mut rng = rand::thread_rng();
            let pick = rng.gen_range(l..r);
            // let pick = rng.gen_range(l, r + 1);
            let pivot = arr[pick];
            swap(arr, pick, r);
            // le: next pos of less/ first item equal to pivot
            // eq: next pos of eq/ first item greater to pivot
            // gt: left next pos of gt/ last item equal to pivot
            let (mut le, mut eq, mut gt) = (l, l, r - 1);
            while eq <= gt {
                // println!{"{:?} {eq}, {gt}", arr};
                if arr[eq] == pivot {
                    eq += 1;
                } else if arr[eq] < pivot {
                    swap(arr, eq, le);
                    eq += 1;
                    le += 1;
                } else {
                    swap(arr, eq, gt);
                    if gt == 0 {
                        break;
                    }
                    gt -= 1;
                }
            }

            swap(arr, eq, r);
            // println!("{:?}  {le} {eq} {gt}", arr);
            (le, eq)
        }
        fn swap(arr: &mut Vec<i32>, i: usize, j: usize) {
            let t = arr[i];
            arr[i] = arr[j];
            arr[j] = t;
        }
        let k = k as usize;
        let n = nums.len();
        qss(&mut nums, 0, n - 1, n - k);

        nums[n - k]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {}
}
