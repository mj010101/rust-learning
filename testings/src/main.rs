fn main() {
    let mut i: i32 = 1;
    loop {
        i = i.checked_mul(10).expect("overflow");
}
}