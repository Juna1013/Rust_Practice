fn main() {
    let a = 10;
    
    {
        let mut a = 20;
        a += 30;
        println!("{}", a);
    }
    
    println!("{}", a);
}
