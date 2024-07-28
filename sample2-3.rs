fn main() {
    let mut a = 10;
    let a_mut_ref = &mut a;
    let a_mut_ref_move = a_mut_ref;
    print!("{}", a_mut_ref_move)
}
