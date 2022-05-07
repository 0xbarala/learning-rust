// Arrays - fixed list where elements are same data type

use std::mem;

pub fn run() {
    let mut numbers: [i32; 4] = [1,2,3,4];

    println!("{:?}", numbers);

    //Get single val
    println!("Value at 0th index : {}", numbers[0]);

    //update value in array
    numbers[2] = 100;
    println!("Value at 2nd index : {}", numbers[2]);

    //array length
    println!("array leng : {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    //Slice entire array
    let slice: &[i32] = &numbers;
    println!("slice: {:?}", slice);

    //Slice custom range
    let slice_partial: &[i32] = &numbers[0..2];
    println!("partially sliced: {:?}", slice_partial);
}