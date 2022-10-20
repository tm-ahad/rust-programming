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
}
