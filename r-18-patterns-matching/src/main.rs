enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    //pattern matching with `let` keyword

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    //destructing
    let (a, b, c) = (1, 2, 3);

    println!("a = {}, b = {}, c = {}", a, b, c);

    struct RGB
    {
        red: u8,
        green: u8,
        blue: u8
    }

    impl RGB
    {
        fn new((r, g, b): (u8, u8, u8)) -> Self
        {
            Self
            {
                red: r,
                green: g,
                blue: b
            }
        }
    }

    let RGB { red: r, green: g, blue: b, .. } = RGB::new((80, 78, 89));

    println!("red - {}, green - {}, blue - {}", r, g, b);

    //Refutability: Whether a Pattern Might Fail to Match

    // let Some(x) = Some(6); //None is not covered matched!

    //pattern matching (important)

    let a = 1;

    match a {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let b = Some(5);
    let y = 10;

    match b
    {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", b),
    }

    println!("at the end: b = {:?}, y = {y}", b);

    let c = 5;

    match c {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let z = 'c';

    match z {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    let color = RGB::new((56, 65, 67));

    match color {
        RGB { red: 56, green, blue } => println!("g = {green}, b = {blue}"),

        /* rest prop */
        RGB { green: 78, blue, .. } => println!("b = {blue}"),
        _ => {}
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
    }

}
