use std::io;

// Exo 1
// fn main() {
//     let mut buf = String::new();
//     let stdin = io::stdin();
//     println!("Type your first name :")
//     stdin.read_line(&mut buf).unwrap();
//     let name = buf.trim();
//     println!("Hello {}!", name.trim());
// }

fn read_str(mut buf: &mut String) -> String {
    let stdin = io::stdin();
    stdin.read_line(&mut buf).unwrap();
    let result = buf.trim();
    result.parse().unwrap()
}

fn main() {
    let mut buf = String::new();
    let mut buf2 = String::new();

    println!("Type your first name :");
    let first_name = read_str(&mut buf);

    println!("Type your last name :");
    let last_name = read_str(&mut buf2);

    println!("Hello {} {}!", first_name, last_name);
}