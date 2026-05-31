use std::io::stdin;

fn main() {
    let mut memory: f64 = 0.0;
    let mut prev_result: f64 = 0.0;

    println!("数字 + - * / 数字を入力：");

    for line in stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        // 空白で分割
        let tokens: Vec<&str> = line.split(char::is_whitespace).collect();
        dbg!(tokens[0]);

        if tokens[0] == "m+" {
            memory += prev_result;
            print_value(memory);
            continue;
        } else if tokens[0] == "m-" {
            memory -= prev_result;
            print_value(memory);
            continue;
        }

        // let left: f64 = tokens[0].parse().unwrap();
        // let right: f64 = tokens[2].parse().unwrap();

        let left = if tokens[0] == "m" {
            memory
        } else {
            tokens[0].parse().unwrap()
        };
        
        let right = if tokens[2] == "m" {
            memory
        } else {
            tokens[2].parse().unwrap()
        };

        let result = match tokens[1] {
            "+" => add(left, right),
            "-" => subtract(left, right),
            "*" => multiply(left, right),
            "/" => divide(left, right),
            _ => {
                unreachable!()
            }
        };
        print_value(result);
        
        prev_result = result;
    }
}

fn print_value(value: f64) {
    println!(" => {}", value);
}

fn add(left: f64, right: f64) -> f64 {
    left + right
}

fn subtract(left: f64, right: f64) -> f64 {
    left - right
}

fn multiply(left: f64, right: f64) -> f64 {
    left * right
}

fn divide(left: f64, right: f64) -> f64 {
    left / right
}
