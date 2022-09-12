/*Let's see*/

pub fn under_stand_shallow_cloning() {
    /*Creating a string*/
    /*str1 and str2 are saved at one address in heap*/

    let str1: String = String::from("Tah-mid");
    // let mut str2 = str1; //we are moving ref of the str 1

    //here we mutating the ref of str1, str2 so str1 will be change

    // str2 = String::from("Fah-mid");

    /*Let's check*/

    // println!("str1={}", str1) //error;

    //so we can copy the string instead of moving the ref.
    //If your from Javascript or python it's like shallow copying

    let mut str2 = str1.clone();

    println!("str2={}", str2);

    str2 = String::from("Fah-mid");

    //but here we change the value of str2 not mutating the ref so str1 will not changed.

    println!("str1={}\nstr2={} ", str1, str2);

}