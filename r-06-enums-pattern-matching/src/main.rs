use crate::r#enum::{DCSuperHero, MarvelSuperHeroes, Spidey, VenomHost};

mod r#enum;

fn main() {
    let spider_man = MarvelSuperHeroes::SpiderMan ( Spidey::Tobey );
    let venom = MarvelSuperHeroes::Venom { host: VenomHost::Eddie };
    let black_adam = DCSuperHero::BlackAdam;

    //matching enum

    match spider_man {
        MarvelSuperHeroes::SpiderMan(c) => println!("{:?} is spider-man", c),
        _ => {}
    }

    match venom {
        MarvelSuperHeroes::Venom(host) => println!("{:?} is venom", host),
        _ => {}
    }

    match black_adam {
        DCSuperHero::BlackAdam => println!("{}", "Black Adam"),
        _ => {}
    }
}
