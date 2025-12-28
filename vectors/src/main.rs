fn ownership() {
    let numbers = vec![1, 2, 3];
    let slice = &numbers[..];
    println!("Slice = {:?}", slice);
    println!("Numbers = {:?}", numbers);
}

fn modifiable() {
    let mut numbers = vec![1, 2, 3];
    let slice = &mut numbers[..];
    slice[0] = 10;
    println!("Slice = {:?}", slice);
    println!("Numbers = {:?}", numbers);
}

fn main() {
    ownership();
    modifiable();
}