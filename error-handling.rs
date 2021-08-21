fn even_test(number: i32) -> Option<i32> {
    if number % 2 == 0 {
        Some(number)
    } else {
        None
    }
}

fn main() {
    println!("This is even: {}", even_test(21).unwrap());
}
