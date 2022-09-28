use crate::another::{mean, sqr};
use crate::arth_operator::{add, devide, minus, multiply};

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
}

pub mod another
{
    use crate::arth_operator::multiply;
    pub use super::arth_operator::{add, devide};

    pub fn mean(x: i32, y: i32) -> f64
    {

        devide(add(x, y) as f64, 2 as f64)
    }

    pub fn sqr(x: i32) -> i32
    {
        multiply(x, x)
    }
}

pub fn main()
{
    let x = 4;
    let y = 5;

    println!("add - {}", add(x, y));
    println!("minus - {}", minus(y, x));
    println!("devide - {}", devide(x as f64, y as f64));
    println!("multiply - {}", multiply(x, y));
    println!("sqr - {}", sqr(y));
    println!("mean - {}", mean(x, y))
}