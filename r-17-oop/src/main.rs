mod lib;

use lib::AveragedCollection;

fn main() {
    let mut a = AveragedCollection::new(vec![1, 2, 5], (8 / 3) as f64);

    println!("{:?}", a.average());

    a.add(6);

    println!("{:?}", a.remove().unwrap()) //last element;
}
