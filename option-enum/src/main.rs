fn divise(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        None
    } else {
        Some(x / y)
    }
}

fn main() {
    let a = 10;
    let b = 2;
    
    let result = divise(a, b);

    match result {
        Some(x) => println!("Resultat : {}", x),
        None => println!("Erreur: Division par zero")
    }
}