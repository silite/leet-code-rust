static COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {}
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
