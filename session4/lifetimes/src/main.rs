fn borrow<'a>(i: &'a i32, j: &'a i32) -> &'a i32 {
    return i;
}

fn main() {
    let n = &32;
    borrow(n, n);
}
