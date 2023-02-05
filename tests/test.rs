#[cfg(test)]
mod test {

    #[test]
    fn test_n01() {
        use leet_code_rust::n01::Solution;
        let a = Solution::two_sum([3, 2, 4].to_vec(), 6);
        println!("{:?}", a);
    }

    #[test]
    fn test_o09() {
        use leet_code_rust::o09::*;
        let mut obj = CQueue::new();
        obj.append_tail(32);
        let ret_2: i32 = obj.delete_head();
        println!("{}, {}", ret_2, obj.delete_head());
    }
}
