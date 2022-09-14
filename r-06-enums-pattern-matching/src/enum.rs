#[derive(Debug)]
pub enum MarvelSuperHeroes {
    SpiderMan(Spidey),
    Venom { host: VenomHost },
    Hulk,
    DeadPool
}
#[derive(Debug)]
pub enum DCSuperHero {
    BlackAdam,
    SuperMan,
    Shazam,
    Flash
}
#[derive(Debug)]
pub enum Spidey {
    Tobey,
    Andrew,
    Tom,
    MilesMorales,
    Flash
}

#[derive(Debug)]
pub enum VenomHost {
    Eddie,
    Tobey,
    Deadpool
}