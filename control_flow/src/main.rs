use std::io;

fn main() {
    // IF_ELSE BLOCK
    let number = 6;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let a: i32 = if number % 3 == 0 { 10 } else { 0 };

    println!("The value of a is: {}", a);
    // END IF_ELSE BLOCK

    // LOOP BLOCK
    // loop {
    //     println!("again");
    // } // infinite loop

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count = 0;

    'counting_up: loop {
        println!("count = {}", count);
        let mut rem = 10;

        'counting_down: loop {
            println!("rem = {}", rem);
            if rem == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            rem -= 1;
        }
        count += 1;
    }

    // while loop but with using "loop" and "break"

    // let mut number = 3;
    // loop {
    //     if number == 0 {
    //         break;
    //     }
    //     println!("{}!", number);
    //     number -= 1;
    // }

    // while loop itself

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");

    // while loop to iterate through an array

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the {}th value is : {}", index, a[index]);
        index += 1;
    }

    // for loop to iterate through an array

    for element in a {
        println!("the value is : {}", element);
    }

    // for loop with range

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("LIFTOFF!!!");

    // END LOOP BLOCK

    // TASKS:
    // 1. Write a program that converts between Fahrenheit and Celsius.

    const NINE_DIV_FIVE: f32 = 9.0 / 5.0;

    println!("Choose convertion: ");
    println!("1. F to C");
    println!("2. C to F");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Please enter valid number");

    let input: u8 = input.trim().parse().expect("Not a number");

    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Please enter valid number");

    let value: f32 = value.trim().parse().expect("Not a number");

    if (input == 1) {
        let value = (value - 32.0) / NINE_DIV_FIVE;
        println!("ªC = {:.2}", value);
    } else if (input == 2) {
        let value = (value * 1.8) + 32.0;
        println!("ªF = {:.2}", value);
    }

    // Fibonacci
    // 1 1 2 3 5 8 13 21
    let mut first = 1;
    let mut second = 1;
    let mut count = 2;

    //1 1
    //1 2
    //2 3
    //3 5

    let mut input = String::new();

    println!("Please input N for Nth fibonacci number.");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    let input: i32 = input.trim().parse().unwrap();

    while count < input {
        let temp = first;
        second = first + second;
        first = second - temp;
        count = count + 1;
    }

    println!("The Nth Fibonacci number is {}!", second);
    println!("Also the Nth Fibonacci number is {}.", nth_fibonacci(input));

    //The Twelve Days of Christmas lyrics

    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    // Twelve drummers drumming,
    // Eleven pipers piping,
    // Ten lords a-leaping,
    // Nine ladies dancing,
    // Eight maids a-milking,
    // Seven swans a-swimming,
    // Six geese a-laying,
    // Five gold rings,
    // Four calling birds,
    // Three French hens,
    // Two turtle doves,
    // And a partridge in a pear tree

    let gifts = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three french hens",
        "four calling birds",
        "five golden rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    for i in 0..12 {
        println!(
            "\nOn the {} day of Christmas,\nmy true love sent to me,",
            days[i]
        );
        for j in (0..=i).rev() {
            if i != 0 && j == 0 {
                print!("And ");
            }
            println!("{}", gifts[j]);
        }
    }
}

fn nth_fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }

    nth_fibonacci(n - 1) + nth_fibonacci(n - 2)
}
