use std::io;

fn main() {
    loop {
        println!("\nВведите 'start' чтобы начать: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();

        if input == "start" {
            break;
        }
    }

    loop {
        println!("\n[1] - Сложение\n[2] - Вычитание\n[3] - Умножение\n[4] - Деление\n[5] - Возведение в степень\n[6] - Процент\n[e] - Выйти");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();

        if input == "e" {
            break;
        }

        let (num1, num2) = match input {
            "1" | "2" | "3" | "4" => get_two_numbers(),
            "5" | "6" => get_numbers(input),
            _ => continue,
        };

        match input {
            "1" => println!("Ответ: {}", num1 + num2),
            "2" => println!("Ответ: {}", num1 - num2),
            "3" => println!("Ответ: {}", num1 * num2),
            "4" => println!("Ответ: {}", num1 / num2),
            "5" => println!("Ответ: {}", num1.powf(num2)),
            "6" => println!("Ответ: {}", (num1 * num2) / 100.0),
            _ => (),
        }
    }
}

fn get_two_numbers() -> (f64, f64) {
    println!("Введите первое число: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num1: f64 = input.trim().parse().expect("Invalid number");

    println!("Введите второе число: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num2: f64 = input.trim().parse().expect("Invalid number");

    (num1, num2)
}

fn get_numbers(operation: &str) -> (f64, f64) {
    match operation {
        "5" => get_two_numbers(),
        "6" => {
            println!("Введите базовое число: ");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            let num1: f64 = input.trim().parse().expect("Invalid number");

            println!("Введите процент: ");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            let num2: f64 = input.trim().parse().expect("Invalid number");

            (num1, num2)
        }
        _ => (0.0, 0.0),
    }
}