fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    
    let third_value = vec[2];
    println!("La troisieme valeur est : {}", third_value);

    let last = vec.last().unwrap();
    println!("La derniere valeur est : {}", last);

    match vec.first() {
        Some(first_value) => println!("La premiere valeur est : {}", first_value),
        None => println!("Le vecteur est vide"),
    }

    let index = 4;
    let value = vec.get(index).unwrap();
    println!("La vaeur a l'index {} est : {}", index, value);
}