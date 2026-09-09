use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guesss.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    // Exercise: 2
    let x = 5;
    let y = 20;
    println!("x = {x} and y + 2 = {}", y + 2);
}
