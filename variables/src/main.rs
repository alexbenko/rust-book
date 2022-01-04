fn main() {
    let x: u32 = 5;
    let x = x + x;
    {
        // inner scope
        let x = x * x;
        println!("Value of x in inner scope: {}", x);
    }
    println!("Value of x in outer scope: {}",x);

    // destructuring with compound types
    // tuples are like arrays but can have different types
    let values = (1, 5.1 , 15);
    let (a, _b, _c) = values;
    println!("A is: {}", a);

    let five = values.1;
    println!("Value at index 1: {}", five);

    //arrays in rust are different besause they have a fixed length and all values must be the same type
    //vectors are a similar data type, vectors are included in the std library and can change length, if unsure use a vector
}
