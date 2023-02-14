fn main() {
    let s = String::from("aaa");
    let y = |x| s + x;
    ex1(y);
}

fn ex1<'a>(ex: impl FnOnce(&'a str) -> String) {
    ex("bbb");
}
