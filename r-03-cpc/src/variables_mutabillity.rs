/* mutable 🆚 immutable variable */

/*mutable means you can change that value anywhere*/

/*immutable means the opposite of mutable you can't change that value*/
/*If your from another lang like TS, JS, Python, immutable like constant*/

pub fn infinite_loop (){
    // let i = 0;
    //
    // loop {
    //     println!("Infinite loop {} ...", i);
    //     i = i + 1; //can't reassign because it is immutable
    // }

    /*How can we make a variable mutable Let's see! */

    /*making a variable mutable with mut keyword*/
    let mut i = 0;

    loop {
        println!("Infinite loop {} ...", i);
        i = i + 1; //can't reassign because it is immutable
    }
}