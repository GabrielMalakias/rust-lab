trait User {
    fn new(username: &'static str) -> Self;
    fn username(&self) -> &'static str;
    fn login(&self) -> &'static str;
    fn logout(&self) -> &'static str;
    fn is_logged_in(&self) -> bool {
        false
    }
}

struct Admin { username: &'static str }
struct Operator { username: &'static str }

#[derive(Debug)]
struct BasicUser { username: &'static str }

impl User for BasicUser {
    fn new(username: &'static str) -> BasicUser {
        BasicUser { username: username }
    }

    fn username(&self) -> &'static str {
        self.username
    }

    fn login(&self) -> &'static str {
        "BasicUser user just logged in"
    }

    fn logout(&self) -> &'static str {
        "BasicUser user just logged out"
    }
}

impl User for Operator {
    fn new(username: &'static str) -> Operator {
        Operator { username: username }
    }

    fn username(&self) -> &'static str {
        self.username
    }

    fn login(&self) -> &'static str {
        "Operator user just logged in"
    }

    fn logout(&self) -> &'static str {
        "Operator user just logged out"
    }
}

impl User for Admin {
    fn new(username: &'static str) -> Admin {
        Admin { username: username }
    }

    fn username(&self) -> &'static str {
        self.username
    }

    fn login(&self) -> &'static str {
        "Admin user just logged in"
    }

    fn logout(&self) -> &'static str {
        "Admin user just logged out"
    }
}

fn main() {
    let admin: Admin = User::new("Max Mustermann");
    println!("Welcome {}", admin.username());
    println!("{}", admin.login());
    println!("{}", admin.logout());

    let user: Admin = User::new("John Doe");
    println!("Welcome {}", user.username());
    println!("{}", user.login());
    println!("{}", user.logout());

    let basic: BasicUser = User::new("Gabriel");
    println!("Welcome {}", basic.username());
    println!("{}", basic.login());
    println!("{}", basic.logout());
    println!("{:?}", basic);
}
