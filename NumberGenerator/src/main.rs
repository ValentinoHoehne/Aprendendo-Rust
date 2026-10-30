use std::cmp::Ordering;
use std::io;
use rand::RngExt;

fn main() {
    limpar_tela();
    println!("=== Jogo da Adivinhação ===");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("\nDigite o seu palpite:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler entrada");

        limpar_tela();

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, digite um número válido!");
                continue;
            }
        };

        println!("Seu último palpite foi: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("🎉 Parabéns! Você acertou!");
                break; // Encerra o jogo ao acertar
            }
        }
    }
}

fn limpar_tela() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}