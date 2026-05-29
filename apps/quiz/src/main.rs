use rand::RngExt;

fn main() {
    println!("===== QUIZ =====");

    let mut rng = rand::rng();
    let op_left = rng.random_range(0..100);
    let op_right = rng.random_range(0..100);

    println!("{} + {} = ??", op_left, op_right);
    println!("?? の値を入力してください:");

    let mut ans_input = String::new();

    std::io::stdin().read_line(&mut ans_input).unwrap();

    let ans_input = ans_input.trim().parse::<u32>().unwrap();

    if ans_input == op_left + op_right {
        println!("正解");
    } else {
        println!("不正解");
    }
}
