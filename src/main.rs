use reqwest::Client;
use std::io::Write;

fn main() {
    print!("digite o link principal da aplicação: ");
    let mut default_link = String::new();
    std::io::stdout().flush();
    std::io::stdin().read_line(&mut default_link).unwrap();
    println!();

    let mut stri = String::new();
    print!(
        "digite o tipo de requisição que se quer fazer
        \x1B[32m1\x1B[0m para \x1B[32mGET\x1B[0m
        \x1B[31m2\x1B[0m para \x1B[31mPOST\x1B[0m
        \x1B[33m3\x1B[0m para \x1B[33mPUT\x1B[0m
        \x1B[35m4\x1B[0m para \x1B[35mDELETE\x1B[0m: "
    );
    std::io::stdout().flush();
    std::io::stdin().read_line(&mut stri).unwrap();

    println!();
    let funcao: u8 = stri.trim().parse().unwrap();
    let client = Client::new();
}
