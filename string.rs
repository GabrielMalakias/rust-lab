fn main(){
    let vec = vec![82, 117, 115, 116];

    let a = String::from_utf8(vec).unwrap();

    let name = "Gabriel".to_string();
    let company = String::from("Delivery Hero");
    let position: String = "Software Engineer".into();

    println!("{:?}", a);

    let mut sentence = name;
    sentence += " works at ";
    sentence += &company;
    sentence += " as ";
    sentence += &position;

    println!("{}", sentence);


    let mut a = String::from("Rust");

    for _x in 0..(a.len() + 1) {
        let r = a.pop();
        match r {
            Some(item) => println!("Pop -> {}", item),
            None => println!("No more chars"),
        }
    }

    let country = "Brazil".to_string();

    for (index, character) in country.chars().enumerate() {
        println!("{} -> {}", index, character)
    }

    // casting
    let z = String::from("7");
    let y = match z.parse::<i8>() {
        Ok(x) => x + 1,
        Err(_) => 0,
    };

    println!("7 + 1 = {}", y);

    // find and divide
    let mut full_name = String::from("Gabriel Malaquias de Souza");
    let space = full_name.find(" ").unwrap_or(0);
    let first_name: String = full_name.drain(..space).collect();

    println!("First name: {}", first_name);
    println!("Surname: {}", full_name);
}
