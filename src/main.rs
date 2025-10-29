use std::io;
use std::io::Write;

// Exo 1
// fn main() {
//     let mut buf = String::new();
//     let stdin = io::stdin();
//     println!("Type your first name :")
//     stdin.read_line(&mut buf).unwrap();
//     let name = buf.trim();
//     println!("Hello {}!", name.trim());
// }

fn read_str(typing_message: &str, buf: &mut String) -> String {
    println!("{}", typing_message);
    io::stdout().flush().unwrap();
    let stdin = io::stdin();
    stdin.read_line(buf).unwrap();
    let result = buf.trim().to_string();
    buf.clear();
    result
}

fn main() {
    let mut buf = String::new();
    let mut buf2 = String::new();

    let first_name = read_str("Type your first name :", &mut buf);
    let last_name = read_str("Type your last name :", &mut buf2);

    println!("Hello {} {}!", first_name, last_name);
}