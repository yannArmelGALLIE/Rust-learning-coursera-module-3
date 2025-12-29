#[derive(Debug)]
enum AttiekeRegions {
    Abidjan,
    Yakro,
    Abengourou,
    Bouake,
}

struct Attieke {
    name: String,
    region : AttiekeRegions,
}

fn supported_regions(a: AttiekeRegions) {
    match a {
        AttiekeRegions::Abidjan => println!("Abidjan est pris en charge"),
        _ => println!("{:?} n'est pas pris en charge", a),
    }
}

fn main() {
    let attieke1 = Attieke {
        name: String::from("Garba"),
        region: AttiekeRegions::Abengourou,
    };

    let attieke2 = Attieke {
        name: String::from("Abodjama"),
        region: AttiekeRegions::Abidjan,
    };

    println!("Attieke1 : {} vient de {:?}", attieke1.name, attieke1.region);
    supported_regions(attieke1.region);
    println!("Attieke2 : {} vient de {:?}", attieke2.name, attieke2.region);
    supported_regions(attieke2.region);
}