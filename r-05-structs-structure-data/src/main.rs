
use crate::r#struct::Person;

mod r#struct;

fn main() {
    //creating a person
    let person = Person {
        name: String::from("Tah-mid"),
        age: 9,
        email: String::from("tm.ahad.bd@gmail.com"),
        phone: 123456789987654
    };

    //creating a constructor

    let person2 = create_person("Fah-mid", 5, "fm.ahad.bd@gmail.com", 45678954458);

    println!("person1 = {:?},\nperson2 = {:?}", person, person2);
}

pub fn create_person(name: &str, age: i8, email: &str, phone: i128) -> Person {
    let str_name = String::from(name);
    let str_email = String::from(email);
    Person {
        name: str_name,
        age,
        email: str_email,
        phone
    }
}