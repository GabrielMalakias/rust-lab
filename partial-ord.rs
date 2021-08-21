use std::cmp::Ordering;

#[derive(PartialOrd, PartialEq, Eq, Copy, Clone)]
struct Person {
    age: i32,
    name: &'static str
}

impl Ord for Person {
    fn cmp(&self, other: &Person) -> Ordering {
        (self.age).cmp(&(other.age))
    }
}

fn older(p1: Person, p2: Person) {
    if p1 > p2 {
        println!("{} is older than {}", p1.name, p2.name);
    } else {
        println!("{} is older than {}", p2.name, p1.name);
    }
}

fn main() {
    let p1 = Person {
        age: 28,
        name: "Gabriel"
    };
    let p2 = Person {
        age: 25,
        name: "Stephani"
    };
    let p3 = Person {
        age: 48,
        name: "Maria"
    };

    older(p1, p2);
    older(p2, p3);
    older(p1, p3);
}
