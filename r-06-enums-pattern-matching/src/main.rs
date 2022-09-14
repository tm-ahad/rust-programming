use crate::r#enum::{MarvelSuperHeroes, Spidey, VenomHost};

mod r#enum;

fn main() {
    let spider_man = MarvelSuperHeroes::SpiderMan(Spidey::Tobey);
    let venom = MarvelSuperHeroes::Venom(VenomHost::Eddie);

    //matching enum

    match spider_man {
        MarvelSuperHeroes::SpiderMan(c) => println!("{:?} is spider-man", c),
        _ => {}
    }

    //You can watch the Venom movie

    match venom {
        MarvelSuperHeroes::Venom(host) => println!("{:?} is venom", host),
        _ => {}
    }

    //which universe's superhero do you like DC or Marvel
    //I like DC 👍👍
}
