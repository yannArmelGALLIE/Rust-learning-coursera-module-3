fn main() {
    let sentence = String::from("the quick brown fox jumps over the lazy dog");
    println!("sentence : {}", &sentence[0..4]);

    let description = format!("Title: Quick story\n{}", sentence);
    println!("description : {}", description);

    let words : Vec<&str> = sentence.split_whitespace().collect();
    println!("{:?}", words);

    let reversed : Vec<char> = sentence.chars().rev().collect();
    println!("{:?}", reversed);
}