use std::io;
fn main() {
    println!("Gib eine Zahl ein");
    let mut input_one = String::new();
    io::stdin().read_line(&mut input_one).expect("Fehler beim Einlesen");
    let number_one: i32 = input_one.trim().parse().expect("Fehler beim Konvertieren");

    println!("Gib eine weitere Zahl ein");
    let mut input_two = String::new();
    io::stdin().read_line(&mut input_two).expect("Fehler beim Einlesen");
    let number_two: i32 = input_two.trim().parse().expect("Fehler beim Konvertieren");

    println!("Gib einen der folgenden Operatoren ein; + - * /");
    let mut input_operator = String::new();
    io::stdin().read_line(&mut input_operator).expect("Fehler beim Einlesen");

    let result = match input_operator.trim() {
        "+" => (number_one + number_two) as f64,
        "-" => (number_one - number_two) as f64, 
        "*" => (number_one * number_two) as f64, 
        "/" => number_one as f64 / number_two as f64, 
        _ => {
            println!("Falscher Operator");
            0.0 
        }
    };

    println!("Das Ergebnis ist: {}", result);

}
