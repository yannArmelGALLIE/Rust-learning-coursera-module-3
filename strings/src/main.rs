fn print_str(s: &str) {
    println!("Slice : {}", s);
}

fn print_string(s: String) {
    println!("String : {}", s);
}

fn main() {
    let s = "Hello, world";
    print_str(&s);
    println!("s : {}", s);

    let string = String::from("Hello");
    print_string(string);
    // println!("string : {}", string);
}