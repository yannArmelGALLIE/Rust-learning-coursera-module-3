#[derive(Debug)]
struct Person {
    nom: String,
    prenom: String,
    age: u8,
}

fn main() {
    println!("{:?}", Person{
        nom: "GALLIE".to_string(),
        prenom: "Koffi Yann-Armel".to_string(),
        age: 19,
    })
}