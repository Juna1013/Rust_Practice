fn main() {
    let a = 10;
    let a_ref = &a;
    let a_ref_copy = a_ref;
    print!("{} {} {}", a, a_ref, a_ref_copy);
}
