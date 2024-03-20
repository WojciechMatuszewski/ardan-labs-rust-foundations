fn read_line() -> String {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).expect("Stdin not working");

    return input.trim().to_string();
}

fn main() {
    let input = read_line();
    println!("You typed: {input}");
}
