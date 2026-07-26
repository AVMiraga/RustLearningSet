fn power(x: i32, n: i32) -> f32 {
    let mut result: f32 = 1.0;

    if n == 0 {
        return 1.0;
    } else {
        for i in 0..i32::abs(n) {
            result *= x as f32;
        }
    }

    if n < 0 {
        result = 1.0 / result;
    }

    result
}

fn main() {
    println!("{}", power(2, -2));
}
