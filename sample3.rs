fn main() {
    let mut a = 10;
    let a_ref1 = &a;
    let a_mut_ref1 = &mut a;
    let a_mut_ref2 = &mut a;
    let a_ref2 = &a;
    println!("{}", a_ref2);
}
