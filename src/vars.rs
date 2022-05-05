pub fn run() {
    //by default vars are immutable in rust
    let name = "cr7";
    // use mut to make variables mutable
    let mut age = 36;
    println!("name :: {} and I'm {} years old", name, age);

    age += 1;
    println!("name :: {} and I'm {} years old", name, age);

    //define constants (need to mention type exxplicitly)
    const ID: i32 = 001;
    println!("id : {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("CR7", 37);
    println!("name :: {} and age :: {}", my_name, my_age);
}