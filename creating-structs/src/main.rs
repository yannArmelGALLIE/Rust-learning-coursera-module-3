#[derive(Debug)]
struct Person {
    nom: String,
    prenom: String,
    age: Option<u8>
}

fn main() {
    let gallie = Person{
        nom: "GALLIE".to_string(),
        prenom: "Koffi Yann-Armel".to_string(),
        age: Some(19),
    };

    println!("Votre nom de famille est : {}", gallie.nom);
    if let Some(x) = gallie.age {
        println!("Votre age est : {}", x);
    }

}