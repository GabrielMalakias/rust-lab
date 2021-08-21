use std::thread;

fn main() {
    let mut value: i32 = 10;

    let a = thread::spawn(move || {
        value = value + 123;
        println!("Value from thread A {}", value);
    });

    let b = thread::spawn(move || {
        value = value + 1;
        println!("Value from thread B {}", value);
    });

    let _ = a.join();
    let _ = b.join();

    println!("Main thread: {}", value);
}
