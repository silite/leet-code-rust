fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);
    // 此处就不能使用 list 变量
    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

#[test]
fn feature() {
    main()
}
