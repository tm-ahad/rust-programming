fn main() {
    // let r;
    //
    // {
    //     let x = String::from("Fah-mid ");
    //     r = &x;
    // }

    // println!("r: {}", r);

    let str1: &str = "Tah-mid";
    let str2: &str = "Fah-mid";

    let res = shortest(str1, str2);

    println!("{}", res);

    //r -> &x -> x
    //but x is destroyed.
}

//error
// pub fn shortest(str1: &str, str2: &str) -> &str
// {
//     //because of the str1 or str2 is will finish after execute the function
//
//     if str1.len() < str2.len()
//     {
//         return str1;
//     }
//     str2
// }


pub fn shortest<'a>(str1: &'a str, str2: &'a str) -> &'a str
{
    //because of the str1 or str2 is will finish after execute the function

    if str1.len() < str2.len()
    {
        return str1;
    }
    str2
}