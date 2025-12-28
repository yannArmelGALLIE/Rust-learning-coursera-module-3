#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
}

impl User {
    fn new(username: String, email: String) -> Self {
        Self {
            username,
            email,
            active: true,
        }
    }

    fn deactivate(&mut self) {
        self.active = false;
    }
}

fn main() {
    let mut gallie = User::new(
        String::from("yannArmelGALLIE"),
        String::from("professional.gallie@gmail.com"),
    );

    println!("Bonjour {}", gallie.username);
    gallie.deactivate();
    println!("Active : {}", gallie.active);
}