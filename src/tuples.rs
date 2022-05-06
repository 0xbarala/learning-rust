// Max 12 elements
//Can have different type of elements


pub fn run() {
    let person: (&str, &str, i8) = ("CR7", "Portugal", 37);

    println!("{} is from {} and is {} old", person.0, person.1, person.2);
}