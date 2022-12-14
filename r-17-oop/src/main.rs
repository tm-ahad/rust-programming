mod lib;
mod r#trait;
mod oop_design_pattern;

use lib::AveragedCollection;
use crate::r#trait::{Draw, Button, Screen};
use crate::oop_design_pattern::Post;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("{}", "Drawing Select box...")
    }
}

fn main() {
    let mut a = AveragedCollection::new(vec![1, 2, 5], (8 / 3) as f64);

    println!("{:?}", a.average());

    a.add(6);

    println!("{:?}", a.remove().unwrap()); //last element;

    //trait

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();


    let mut post = Post::new(String::new());

    post.add("DC universe is coming with a new villain !!Black Adam!!".to_string());

    println!("{}", post.content());
    post.update("Marvel is coming with a new movie !!Venom 3!!".to_string());

    println!("{}", post.content());

    post.post();

}
