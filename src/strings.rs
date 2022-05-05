
/**
 * Primitive Str = Immutable fixed length string
 * 
 * String = Growable, heap allocated data structure
 * 
 */

pub fn run() {
    let mut hello = String::from("Hello ");

    println!("{}", hello);
    println!("length {}", hello.len());

    //push char to string
    hello.push('W');
    println!("{}", hello);
    println!("length {}", hello.len());

    //push string
    hello.push_str("orld");
    println!("{}", hello);
    println!("length {}", hello.len());

    //capcity in bytes
    println!("Capacity: {}", hello.capacity());

    // is empty
    println!("isEmpty {}", hello.is_empty());

    //contains sub string
    println!("contains word `Wor` {}", hello.contains("Wor"));

    //replace substring
    println!("replace `World` with `There` :: {}", hello.replace("World", "There"));

    //loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    //create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    //Assertion testing
    //assert_eq!(3, s.len());
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
}