use std::io::Write;

#[derive(Debug)]
struct Data {
    name: String
}

fn main() {
    println!("Hello, world!");

    let data = vec![1, 2, 3, 4, 5];

    println!("vev: {:?}", data);

    println!("data: {:?}", Data { name: "n1".to_string() });
}
