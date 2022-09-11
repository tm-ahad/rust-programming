
/*recursion means a function calls in the function through a condition*/
/*let's see.*/

pub fn fact(val: i32) -> i32
{
    // if val 0 or 1 it's return 1

    if val == 0 || val == 1
    {
        return 1;
    }

    //recursion
    return val * fact(val - 1);
}