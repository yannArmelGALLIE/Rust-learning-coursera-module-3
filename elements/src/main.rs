fn main() {
    let mut vec = vec![1, 2, 3];
    vec.push(4);
    println!("{:?}", vec);

    let more = vec![5, 6];
    vec.extend(more);
    println!("{:?}", vec);

    let mut others = vec![7, 8];
    vec.append(&mut others);
    println!("{:?}", vec);

    vec.insert(0, 0);
    println!("{:?}", vec);
}