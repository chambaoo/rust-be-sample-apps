use rand::RngExt;

fn main() {
    println!("===== QUIZ =====");

    let mut rng = rand::rng();
    let quiz_mode = rng.random_range(1..=2);
    let op_left = rng.random_range(0..100);
    let op_right = rng.random_range(0..100);

    match quiz_mode {
        1 => {
            println!("{} + {} = ??", op_left, op_right);
        }
        2 => {
            println!("{} - {} = ??", op_left, op_right);
        }
        _ => unreachable!(),
    }
    println!("?? の値を入力してください:");

    let mut ans_input = String::new();

    std::io::stdin().read_line(&mut ans_input).unwrap();

    let ans_input = ans_input.trim().parse::<u32>().unwrap();

    match quiz_mode {
        1 => {
            if ans_input == op_left + op_right {
                println!("正解");
            } else {
                println!("不正解");
            }
        }
        2 => {
            if ans_input == op_left - op_right {
                println!("正解");
            } else {
                println!("不正解");
            }
        }
        _ => unreachable!(),
    }
}
