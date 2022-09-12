

pub fn under_stand_slice() {
    let mut a: String = String::from("Tah-mid");
    let mut slice1 = &a[0..3];
    let mut slice2 = &a[4..7];

    println!("a={}\nslice1={}\nslice2={}\n\nafter changing\n", a, slice1, slice2);

    //after changing slices
    a.clear();
    slice1 = "Fah";
    slice2 = "-mid";

    println!("a={:?}\nslice1={}\nslice2={}", a, slice1, slice2);
}