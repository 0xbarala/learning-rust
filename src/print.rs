pub fn run() {
    println!("hello from print.rs file!");

    //formatting example
    println!("Number :: {}", 1);
    println!("{} == {}", 1, 1);

    //Positional arguments
    println!("{0} is cool {1} but {0} is difficult to leanr", "Rust", "language");

    //Named arguments
    println!("{name} likes to play {activity}",
        name = "CR7",
        activity = "Football");


    //Placeholder traits
    println!("binary : {:b}, Hex {:x}, Octal: {:o}", 10, 10, 10);

    //Placeholder for debug trait
    println!("{:?}", (12, true, "Hello"));

    //Basic maths
    println!("10 + 10 = {}", 10 + 10);
}