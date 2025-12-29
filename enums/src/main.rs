#[derive(Debug)]
enum DiskType {
    SSD,
    HDD,
}

#[derive(Debug)]
enum DiskSize {
    KB(u32),
    MB(u32),
    GB(u32),
}

fn main() {
    let disk_type = DiskType::SSD;
    println!("{:?}", disk_type);

    let disk_size = DiskSize::GB(128);
    if let DiskSize::GB(x) = disk_size {
        println!("{}", x);
    }
}