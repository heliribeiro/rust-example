use std::io;
fn main() {
    println!("Digite um multiplicador");
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let multi: i32 = number
        .trim()
        .parse()
        .expect("error when converting to number");


    for number in 0..=9 {
        let result = number * multi;
        println!("{number} x {multi} = {}", result);
    }
}
