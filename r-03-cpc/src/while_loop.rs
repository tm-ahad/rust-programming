
/*Let's see how to implement while loop*/
/*Sometimes we can us while loop instead of infinity loop*/

pub fn while_loop(vec: Vec<&str>, index: i32)
{
    let mut ind = index as usize;

    //loop is rendering till the condition in false
    while ind != 0
    {

        println!("{}", vec[ind]);
        ind = ind - 1;
    }
 }