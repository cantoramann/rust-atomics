pub fn example8_main() {
    let a = 1;
    let mut b = 1;

    f(&a, &mut b);
}

fn f(a: &i32, b: &mut i32) {
    let before = *a;
    *b += 1;
    let after = *a;

    println!("Before: {}, After: {}", before, after);
}
