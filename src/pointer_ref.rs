//Reference pointers - Point to a resource in memory

pub fn run() {
    //primitive arr
    let arr1 = [1,2,3];
    let arr2 = arr1;
    println!("Values : {:?}", (arr1, arr2));

    //with non-primitive, if you assign another variable to a piece of data, the first varaible will no longer hold the value,
    //You will need to use reference (&) to point to the resource

    let vec1 = vec![1,2,3];
    let vec2 = &vec1;
    let vec3 = vec2;

    println!("Values: {:?}", (&vec1, vec2, vec3));
}