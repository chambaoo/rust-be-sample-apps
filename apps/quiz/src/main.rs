fn main() {
    println!("===== QUIZ =====");
    println!("1 + 1 = ??");
    println!("?? の値を入力してください:");

    let mut ans_input = String::new();

    std::io::stdin().read_line(&mut ans_input).unwrap();

    let ans_input = ans_input.trim().parse::<u32>().unwrap();

    dbg!(ans_input);

    if dbg!(ans_input == 1 + 1) {
        println!("正解");
    } else {
        println!("不正解");
    }
}
