fn copy_trait_check<T: Copy>(_: T) {}

fn main() {
    let a = 10;
    copy_trait_check(a);
}
