
/**
 * Primitive types
 * 
 * i8, i16, i32, i64, i128, u8, u16, u32, u64, u128 (number if bits in memory)
 * Floats :: f32, f64
 * Boolean (bool)
 * Characters (char)
 * Tuples
 * Arrays
 */

pub fn run() {
    //default i32
    let _x = 1;

    //default if f64
    let _y = 1.1;

    //add explicit type
    let _z: i64 = 123421442141233123;

    //Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //boolean
    let is_active = true;
    let is_greater: bool = 10 > 5;
    
    println!("{:?}", (_x, _y, _z, is_active, is_greater));

    //char
    let _a = 'a';
    let face = '\u{1F600}';
    println!("{:?}", (_x, _y, _z, is_active, is_greater, _a, face));

}