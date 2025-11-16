use stasko_calculator_parser::{CalculatorError, build_ast};
use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        None => {
            run();
        }
        Some("--file") => {
            if let Some(filename) = args.get(2) {
                run_from_file(filename);
            } else {
                eprintln!("Прапор '--file' потребує імені файлу.");
                show_help();
            }
        }
        Some("--help") | Some("-h") => {
            show_help();
        }
        Some("--credits") => {
            show_credits();
        }
        Some(unknown) => {
            eprintln!("Невідомий аргумент '{}'", unknown);
            show_help();
        }
    }
}

/// Функція для запуску з файлу
fn run_from_file(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(input) => {
            let input_trimmed = input.trim();
            if input_trimmed.is_empty() {
                eprintln!("Файл '{}' порожній.", filename);
                return;
            }
            println!("Вираз: {}", input_trimmed);
            match parse_and_eval(input_trimmed) {
                Ok(result) => println!("Результат: {}", result),
                Err(e) => eprintln!("Помилка обчислення: {}", e),
            }
        }
        Err(e) => {
            eprintln!("Не вдалося прочитати файл '{}': {}", filename, e);
        }
    }
}

fn run() {
    println!("Введіть вираз або 'exit' для виходу.");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Помилка читання рядка");
            continue;
        }
        let input = input.trim();
        if input.is_empty() {
            continue;
        }
        if input == "exit" || input == "e" {
            break;
        }
        match parse_and_eval(input) {
            Ok(result) => println!("Результат: {}", result),
            Err(e) => eprintln!("{}", e),
        }
    }
}

/// Функція, що виконує побудову AST та обчислення
fn parse_and_eval(input: &str) -> Result<i64, CalculatorError> {
    let ast = build_ast(input)?;
    let result = ast.eval()?;
    Ok(result)
}

fn show_help() {
    println!(" ");
    println!("Calculator Parser");
    println!("\nВикористання:");
    println!("  cargo run                \t- Запуск інтерактивного режиму.");
    println!("  cargo run -- --file expression.txt\t- Обчислення виразу з файлу expression.txt.");
    println!("  cargo run -- --help      \t- Показати це повідомлення.");
    println!("  cargo run -- --credits   \t- Показати інформацію про автора.");
    println!("\nАбо використовуйте команди Makefile:");
    println!("  make run");
    println!("  make run-file");
    println!("  make help");
    println!("  make credits");
    println!("  make test");
}

fn show_credits() {
    println!("Calculator parser - 2025 Taras Stasko");
}
