use std::io::{self, Write};

// Define the Operation enum with variants for basic arithmetic operations
#[derive(Debug)]
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

// Function to calculate result based on the operation
fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(x, y) => x + y,
        Operation::Subtract(x, y) => x - y,
        Operation::Multiply(x, y) => x * y,
        Operation::Divide(x, y) => {
            if y != 0.0 {
                x / y
            } else {
                println!("Hata: Sıfıra bölme hatası!");
                0.0
            }
        }
    }
}

// Function to get user input
fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    println!("Basit Hesap Makinesi");
    println!("--------------------");

    // Get first number
    let num1: f64 = loop {
        if let Ok(num) = get_input("İlk sayıyı girin: ").parse() {
            break num;
        } else {
            println!("Geçersiz sayı! Lütfen tekrar deneyin.");
        }
    };

    // Get operation
    let operation = loop {
        let op = get_input("İşlemi seçin (+, -, *, /): ");
        match op.as_str() {
            "+" | "-" | "*" | "/" => break op,
            _ => println!("Geçersiz işlem! Lütfen +, -, * veya / kullanın."),
        }
    };

    // Get second number
    let num2: f64 = loop {
        if let Ok(num) = get_input("İkinci sayıyı girin: ").parse() {
            break num;
        } else {
            println!("Geçersiz sayı! Lütfen tekrar deneyin.");
        }
    };

    // Create operation enum instance
    let operation = match operation.as_str() {
        "+" => Operation::Add(num1, num2),
        "-" => Operation::Subtract(num1, num2),
        "*" => Operation::Multiply(num1, num2),
        "/" => Operation::Divide(num1, num2),
        _ => unreachable!(),
    };

    // Calculate and print result
    let result = calculate(operation);
    println!("\nSonuç: {}", result);
}
