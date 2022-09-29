use crate::arth_operator::another;

pub fn another_func(b: i32) -> u32
{
    b as u32
}

pub mod arth_operator
{

    pub fn add(x: i32, y: i32) -> i32
    {
        x + y
    }

    pub fn minus(x: i32, y: i32) -> i32
    {
        x - y
    }

    pub fn devide(x: f64, y: f64) -> f64
    {
        x / y
    }

    pub fn multiply(x: i32, y: i32) -> i32
    {
        x * y
    }

    pub fn mean(x: i32, y: i32) -> f64
    {

        devide(add(x, y) as f64, 2 as f64) as f64
    }

    pub fn another(a: i32) -> u32
    {
        let i = super::another_func(a);
        i
    }
}
pub mod another
{
    pub use super::arth_operator::{multiply};

    pub fn sqr(x: i32) -> i32
    {
        multiply(x, x.clone())
    }
}

pub fn main()
{
    use arth_operator::{add, minus, devide, multiply, mean};
    use another::sqr;

    let x = 6;
    let y = 5;

    println!("add - {}", add(x, y));
    println!("minus - {}", minus(y, x));
    println!("devide - {}", devide(x as f64, y as f64));
    println!("multiply - {}", multiply(x, y));
    println!("sqr - {}", sqr(y));
    println!("mean - {}", mean(x, y));
    println!("another - {}", another(x));
    println!("another_func - {}", another_func(x));
}