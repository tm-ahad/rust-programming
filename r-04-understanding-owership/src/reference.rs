/*But what happened while we want to mutating ref?*/

/*So we can borrow reference but we could not print it. It will be throw a error 🙁🙁🙁*/
/*So we are going to use references 😎*/

/*Let's see*/

pub fn under_stand_reference() {
    /*Creating a string*/
    let mut str1 = String::from("Tah-mid");

    /*Creating a mut ref*/
    let str_ref: &mut String = &mut str1;

    //mutating the ref
    *str_ref = String::from("Fah-mid");

    println!("str1={}\n", str1)
}