fn main() {
    println!("Hello, world!");

    another_function(); // Calling another function
    parameterized_function(5); // Calling function with parameter
    print_labeled_measurement(10, 'm'); // Calling function with multiple parameters

    let y = {
        let x = 5;
        x + 1
    }; // Using a block to assign a value to y
    // Note: No semicolon after the block to return the value
    // Results in y being 6

    println!("The value of y is: {y}");
    let z = five(); // five() returns 5
    println!("The value returned by five() is: {z}");
    let z = add_one(z); // add_one adds 1 to z
    println!("The value after adding one is: {z}");
    let z = summer(z, y); // summer adds z and y
    println!("The final value after summing is: {z}");
}

fn another_function() {
    println!("This is another function.");
}

fn parameterized_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The height is: {}{}", value, unit_label);
}

fn five() -> i32 {
    5
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn summer(a: i32, b: i32) -> i32 {
    a + b
}
