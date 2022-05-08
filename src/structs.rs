//Structs - used to create custom data types

// Traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

//Tuple struct
struct TupleColor(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    //Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    fn name_to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };

    println!("Color: {}, {}, {}", c.red, c.green, c.blue);

    c.red = 111;
    c.green = 200;
    println!("Color: {}, {}, {}", c.red, c.green, c.blue);

    let mut tc = TupleColor(255, 0, 0);
    println!("TupleColor: {}, {}, {}", tc.0, tc.1, tc.2);

    tc.0 = 111;
    tc.1 = 200;
    println!("TupleColor: {}, {}, {}", tc.0, tc.1, tc.2);

    //custom person type
    let mut p = Person::new("V", "B");
    println!("Person:: {} {}", p.first_name, p.last_name);
    println!("Person:: {}", p.full_name());

    //update the last name
    p.last_name = "fdsfd".to_string();
    println!("Person:: {} {}", p.first_name, p.last_name);
    println!("Person:: {}", p.full_name());

    //update the last name
    p.set_last_name("C");
    println!("Person:: {} {}", p.first_name, p.last_name);
    println!("Person:: {}", p.full_name());

    println!("Person as tuple :: {:?}", p.name_to_tuple());
}