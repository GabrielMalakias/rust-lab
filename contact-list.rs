use std::cmp::Ordering;

#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, Debug)]
struct Contact {
    name: &'static str,
    number: &'static str
}

impl Ord for Contact {
    fn cmp(&self, other: &Contact) -> Ordering {
        (self.name).cmp(&(other.name))
    }
}

fn build() -> Vec<Contact> {
    let contact_2 = Contact {
        name: "Gabriel",
        number: "12938182038921"
    };
    let contact_1 = Contact {
        name: "Stephani",
        number: "329483249"
    };
    let contact_3 = Contact {
        name: "Helena",
        number: "9283482903"
    };

    vec![contact_2, contact_1, contact_3]
}

fn main() {
    let mut list = build();
    println!("List with 3 contacts {:?}", list);

    // let contact_3 = Contact {
    //     name: "Maria",
    //     number: "80823840392"
    // };
    // list.push(contact_3);
    // println!("List with 1 contacts {:?}", list);

    // let _ = list.pop();
    // println!("List with 1 contact {:?}", list);
    //

    // println!("Contact {:?}", list[0]);
    // println!("Contact {:?}", list[1]);
    //

    // let names = list.iter().map(|contact| { contact.name }).collect::<Vec<_>>();
    // println!("names > {:?}", names);

    // let numbers = list.iter().map(|contact| { contact.number }).collect::<Vec<_>>();
    // println!("numbers > {:?}", numbers);

    // for contact in list {
    //     println!("---> {:?}", contact);
    // }

    list.sort();

    for contact in list {
        println!("---> {:?}", contact);
    }
}
