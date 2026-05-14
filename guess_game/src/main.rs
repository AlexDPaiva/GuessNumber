use ::std;

fn main() {
    println!("----- Guess Game -----");
    println!("Digite um numero: ");

    let mut guess = String::new();
    std::io::stdin()
        .read_line(&mut guess)
        .expect("Fala ao ler o numero");

    println!("Seu numero foi: {}", guess);
}
