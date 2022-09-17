

fn main() {
    println!("{}", String::from("السلام عليكم"));
    println!("{}", String::from("Dobrý den"));
    println!("{}", String::from("Hello"));
    println!("{}", String::from("שָׁלוֹם"));
    println!("{}", String::from("नमस्ते"));
    println!("{}", String::from("こんにちは"));
    println!("{}", String::from("안녕하세요"));
    println!("{}", String::from("你好"));
    println!("{}", String::from("Olá"));
    println!("{}", String::from("Здравствуйте"));
    println!("{}", String::from("Hola"));

    let mut changeable = String::from("changeable");
    let extra_1 = " extra_1";

    changeable.push_str(extra_1);
    println!("{}", changeable);

    println!("the ownership is not passed {}", extra_1);

    changeable.push(' ');
    changeable.push('+');
    changeable.remove(0);

    let string2 = changeable.replace('a', "e");

    println!("{}, {}", changeable, string2);

    for i in 1..9
    {
        match changeable.chars().nth(i)
        {
            Some(val) => println!("{}", val),
            None => println!("{}", "None".to_string())
        }
    }

}
