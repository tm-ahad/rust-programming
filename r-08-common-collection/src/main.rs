

fn main() {
    //String
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

    //vector
    println!("=====================Vector======================");

    let v = vec![1, 2, 3];

    let mut v2 = Vec::new();

    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);

    println!("{:?}\n{:?}\n", v, v2);

    let v3 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v3[2];

    //If there are no value at the index it's panic
    println!("The third element is {}", third);

    //It's a Option
    let third: Option<&i32> = v.get(2);
    match third
    {
        Some(third) => println!("The third element is {}.\n", third),
        None => println!("There is no third element.\n"),
    }

    let mut v3 = vec![100, 32, 57];

    v3.clear();

    println!("v3 = {:?}", v3);

    v3 = vec![100, 32, 58];
    v3.reverse();
    v3.remove(1);
    v3.push(78);
    v3.push(87);
    v3.repeat(3);

    println!("v3 = {:?}\n", v3);

    for i in &mut v3 {
        *i += 50;

        println!("i - {}", i);
    }
    println!("{}\n", "");

    //There are many Methods available in vector-
    //reverse
    //remove
    //replace
    //repeat

    //HashMap

    use std::collections::HashMap;

    let mut scores = HashMap::new();
    let print_hash_map = |mut hash_map: HashMap<String, i32>| {

        for (key, value) in &mut hash_map {
            println!("h {} -> {}", key, value);
        }
    };

    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Green")).or_insert(80);
    scores.remove("Blue");


    print_hash_map(scores);
}
