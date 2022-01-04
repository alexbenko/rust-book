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

    // A function called drop, and the author of String can put the code to return the memory.
    // Rust automatically calls the drop function and cleans up the heap memory for that variable

    let foo = String::from("foobar");
    let len = calc_length(&foo); // the & syntax that creates a reference, but does not own it.the value it points to will not be dropped when the reference stops being used.

    //mutable references, can only have one reference at a time

    //SLICES
    //Another data type that does not have ownership is the slice. Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection

    let s = String::from("hello world");

    let _hello = &s[0..5];
    let _world = &s[6..11];
    //Rather than a reference to the entire String, it’s a reference to a portion of the String

    let mut aaa = String::from("Hello World");
    let first = first_word(&aaa);
    println!("{}", first); // prints Hello
    //arrays also have slices
}

//If we have a string slice, we can pass that directly. If we have a String, we can pass a slice of the String or a reference to the String
fn first_word(s: &str) -> &str {
    // iter is a method that returns each element in a collection
    // enumerate wraps result of iter and returns each element as part of a tuple
    // The first element of the tuple returned from enumerate is the index, and the second element is a reference to the element
    let bytes = s.as_bytes();
    println!("{:?}", bytes);

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    // returns slice of string if no space
    &s[..]
}

fn calc_length(s: &String) -> usize {
    s.len()
}