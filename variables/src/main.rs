use std::io;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let x: i8 = 127;
    let xf = 2.0;

    let xf = 2.0f32;

    let x = x.saturating_add(1);

    {
        let x = x.saturating_add(2);
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!(
        "{} {} {} {} {} {}",
        sum, difference, product, quotient, truncated, remainder
    );

    let c = 'z';
    let z: char = 'ℤ';
    let opt = '≥';
    let heart_eyed_cat = '😻';

    println!("{} {} {} {}", c, z, opt, heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1); //Also works without explicitly defining type
    let empty_tup = ();
    let (x, y, z) = tup;
    println!("{} {} {}", tup.0, tup.1, tup.2);
    println!("{} {} {}", x, y, z);

    let a = [1, 2, 3, 4, 5];
    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    // let a = [3; 5]; //[3, 3, 3, 3, 3]
    // let a: [i32; 5] = [1, 2, 3, 4]; //exptected : 5, found : 4

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("{} {}", months[0], months[1]);

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("a[{}] = {}", index, element);
}
