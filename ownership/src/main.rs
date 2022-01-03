fn main() {
    // ownership is rust's central feature, it is an alternative to garbage collection
    let _s = "String"; // this is a string literal and it cannot be mutated
    // it is more effecient

    // rust has an alternate type, String
    //This type manages data allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time
    let mut foo = String::from("Foo");
    //The double colon (::) is an operator that allows us to namespace this particular from function under the String type rather than using some sort of name like string_from

    foo.push_str("Bar"); // this works
    println!("{}", foo); //prints FooBar
    //s.push_str("")  this wont work as mentioned above

    //Ownership example:
        // In rust , memory is automatically returned once the variable that owns it goes out of scope

    {
        let mut _score = 100; // score is valid only in this code block
        // work with score

    } // score is no longer valid after this bracket

    // A function is called drop, and it’s where the author of String can put the code to return the memory.
    // Rust automatically calls the drop function and cleans up the heap memory for that variable

}
