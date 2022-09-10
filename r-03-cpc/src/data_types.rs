

pub fn dt() {
   let a: i8 = 9;
   let b: i16 = 11;
   let c: i32 = 20;

   /*There are 3 string types. We will discuss the difference in another chapter*/

   let d: &str = "Tah-mid";
   // let e: String = String::from("Tah-mid");
   //or
   let e: String = "Tah-mid".to_string();

   let pound: char = '£';

   let f: f32 = 10000.00;
   let g: f64 = 25000.00;

   println!("{}: {}", "Warning⚠", "/n means the remaining part will print in a new line\n");

   println!("a = {},\nb = {},\nc = {},\nd = {},\ne = {},\npound = {}, \nf = {}, \ng = {}
   ", a, b, c, d, e, pound, f, g);

   println!("{}", "Working good!");

}