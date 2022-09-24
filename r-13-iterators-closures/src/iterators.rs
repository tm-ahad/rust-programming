

pub fn understand_iterators()
{
    let v = vec![1, 2, 7, 4, 5, 6];
    let mut it = v.iter();

    let v2: Vec<i32> = v
        .iter()
        .map(|v| v*v)
        .collect();

    println!("{:?}", v2);

    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());
}