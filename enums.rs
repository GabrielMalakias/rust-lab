enum Gender {
    Female,
    Male
}

struct Person {
    name: &'static str,
    gender: Gender
}

fn main() {
    let gabriel = Person {
        name: "Gabriel Malaquias",
        gender: Gender::Male
    };

    let stephani = Person {
        name: "Stephani Mendes",
        gender: Gender::Female
    };
}
