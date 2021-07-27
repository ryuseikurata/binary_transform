fn main() {
    let decimal_number = 50;
    let binary = 2;

    let mut result_string = "".to_string();

    let mut prev_n = decimal_number;
    let mut n = decimal_number / binary;
    while n != 1 {
        let result = prev_n - n * binary;
        println!("{}", result);
        result_string.push_str(&result.to_string());
        prev_n = n;
        n = n / binary;
    }
    // 1になった時の処理
    let last_str = prev_n - n * binary;
    result_string.push_str(&last_str.to_string());

    // 1を最後に追加
    result_string.push_str("1");

    println!("{}", reverse_string(&result_string))
}

fn reverse_string(input: &String) -> String {
    let mut reversed = String::new();
    let mut chars: Vec<char> = Vec::new();

    for c in input.chars().into_iter() {
        chars.push(c);
    }

    for i in (0..chars.len()).rev() {
        reversed += &chars[i].to_string();
    }

    return reversed;
}
