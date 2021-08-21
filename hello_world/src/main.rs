extern crate time;

fn main() {
    const THE_1990: i32 = 1900;
    const THE_CHAR: char = '\u{23F3}';

    let date = time::now();
    let day: i32 = date.tm_mday;
    let month: i32 = date.tm_mon;
    let year: i32 = date.tm_year + THE_1990;

    println!("{} Today is {}/{}/{}", THE_CHAR, day, month, year);
}
