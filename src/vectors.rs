// Vectors - resizable arrays where elements are same data type

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4];

    println!("{:?}", numbers);

    //Get single val
    println!("Value at 0th index : {}", numbers[0]);

    //update value in vector
    numbers[2] = 100;
    println!("Value at 2nd index : {}", numbers[2]);

    //array length
    println!("array leng : {}", numbers.len());

    // Vectors are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    //Slice entire vector
    let slice: &[i32] = &numbers;
    println!("slice: {:?}", slice);

    //Slice custom range
    let slice_partial: &[i32] = &numbers[0..2];
    println!("partially sliced: {:?}", slice_partial);

    //add elements to vector
    numbers.push(5);
    println!("{:?}", numbers);

    numbers.push(6);
    println!("{:?}", numbers);

    //pop last element
    numbers.pop();
    println!("{:?}", numbers);

    //loop through vector
    for x in numbers.iter() {
        println!("{}", x);
    }

    //loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec : {:?}", numbers);
}