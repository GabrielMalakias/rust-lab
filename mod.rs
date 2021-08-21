mod math {
    fn is_zero(number: i32) -> bool {
        if number == 0 { return true };
        false
    }

    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn divide(a: i32, b: i32) -> i32 {
        if is_zero(b) { return 0 }
        a / b
    }

    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }

    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }
}

fn check_grade(grade: f32) -> () {
    match grade {
        0.0..=4.8 => println!("Disapproved"),
        4.9..=5.9 => println!("Exam"),
        5.9..=10.0 => println!("Approved"),
        _ => println!("Invalid!"),
    }
}

// testing aliases
use math::add as m_add;
use math::subtract as m_sub;
use math::divide as m_div;
use math::multiply as m_mul;

fn main() {
    let a: i32 = 10;
    let b: i32 = 4;

    println!("{} + {} = {}", a, b, m_add(a, b));
    println!("{} - {} = {}", a, b, m_sub(a, b));
    println!("{} / {} = {}", a, b, m_div(a, b));
    println!("{} * {} = {}", a, b, m_mul(a, b));


    check_grade(0.0);
    check_grade(3.2);
    check_grade(5.1);
    check_grade(8.3);
    check_grade(11.0);
}


