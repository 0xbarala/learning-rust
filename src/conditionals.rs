
pub fn run() {
    let age: u8 = 18;
    let check_id: bool = false;
    let knows_person_of_age = true;

    //if else
    if age >= 21 && check_id || knows_person_of_age {
        println!("What would u like to drink?");
    } else if age < 21 && check_id {
        println!("Sorry, u have to leave");
    } else {
        println!("will need to see your Id");
    }

    //shorthand if
    let is_of_age = if age >= 21 { true } else { false };
    println!("is of age? {}", is_of_age);
}