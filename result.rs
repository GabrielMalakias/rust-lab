use std::fs::File;
use std::io::prelude::*;
use std::io;

struct Info {
    name: &'static str,
    age: i32,
    rating: i32,
}

fn write_info(info: &Info) -> io::Result<()> {
    let mut file = match File::create("testing.txt") {
        Err(e) => return Err(e),
        Ok(f) => f,
    };

    try!(file.write_all(format!("name: {}\n", info.name).as_bytes()));
    try!(file.write_all(format!("age: {}\n", info.age).as_bytes()));
    try!(file.write_all(format!("rating: {}\n", info.rating).as_bytes()));
    Ok(())
}

fn main() {
    let info = Info {
        name: "Gabriel Malaquias",
        age: 93,
        rating: 4
    };

    match write_info(&info) {
        Err(e) => println!("Ops, something wrong -> {}", e),
        Ok(()) => println!("All good"),
    };
}
