fn main() {
    let x: u32 = 5;
    let x = x + x;
    {
        let x = x * x;
        println!("Value of x in inner scope: {}", x);
    }
    println!("Value of x in outer scope: {}",x);
}
