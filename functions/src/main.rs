fn main() {
    greet();
    println!("{}", add_two(100,312));

    let foo = {
        let a = 12;
        let b = 18;
        add_two(a, b)
    };
    println!("Foo: {}", foo);
}

fn greet(){
    println!("Hello, world!");
}

fn add_two(x: u32, y: u32) -> u32{
    x + y
    // same as: return x + y;
    // If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value
}