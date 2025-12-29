use reqwest::blocking::Client;
use std::io::Write;
fn main() {
    print!("digite o link principal da aplicação: ");
    let mut rota = String::new();
    let _ = std::io::stdout().flush();
    std::io::stdin().read_line(&mut rota).unwrap();
    println!();

    let mut stri = String::new();
    print!(
        "digite o tipo de requisição que se quer fazer
        \x1B[32m1\x1B[0m para \x1B[32mGET\x1B[0m
        \x1B[31m2\x1B[0m para \x1B[31mPOST\x1B[0m
        \x1B[33m3\x1B[0m para \x1B[33mPUT\x1B[0m
        \x1B[35m4\x1B[0m para \x1B[35mDELETE\x1B[0m: "
    );
    let _ = std::io::stdout().flush();
    std::io::stdin().read_line(&mut stri).unwrap();

    println!();
    let funcao: u8 = stri.trim().parse().unwrap();
    let client = Client::new();
    let subrota = String::new();
    print!("digite a subrota: ");
    let _ = std::io::stdout().flush();
    std::io::stdin().read_line(&mut rota).unwrap();
    println!();
    let retorno = match funcao {
        1 => client
            .get(rota + subrota.as_str())
            .send()
            .unwrap()
            .text()
            .unwrap(),
        2 => {
            print!("digite o json: ");
            let mut jason = String::new();
            let _ = std::io::stdout().flush();
            std::io::stdin().read_line(&mut jason).unwrap();
            let jason = jason.trim();
            println!();
            client
                .post(rota + subrota.as_str())
                .json(&serde_json::from_str::<serde_json::Value>(jason).unwrap())
                .send()
                .unwrap()
                .text()
                .unwrap()
        }
        _ => "erro".to_string(),
    };
    println!("{retorno}");
}
