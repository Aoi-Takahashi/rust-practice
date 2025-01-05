/// NO.00:実行するとHello World!と表示するプログラムを作成せよ。
pub fn no_00() {
    println!("Hello, world!");
}
#[test]
fn test_no_00() {
    assert_eq!(no_00(), println!("Hello, world!"))
}

/// NO.01:12345+23456を計算して結果を表示するプログラムを作成せよ。
pub fn no_01() -> u32 {
    let sum = 12345 + 23456;
    println!("12345 + 23456={}", &sum);
    return sum;
}
#[test]
fn test_no_01() {
    assert_eq!(no_01(), 12345 + 23456);
}

/// NO.02:12345を7で割った余りを表示するプログラムを作成せよ。
pub fn no_02() -> u32 {
    let result = 12345 % 7;
    println!("12345 / 7={}", &result);
    return result;
}
#[test]
fn test_no_02() {
    assert_eq!(no_02(), 4)
}

/// NO.03:整数値を入力させ、その入力値を表示するプログラムを作成せよ。
pub fn no_03() -> u32 {
    use std::io;
    let mut buf = String::new();
    println!("数字を入力してください");
    io::stdin()
        .read_line(&mut buf)
        .expect("標準入力の読み取りに失敗しました");
    let input_number = buf.trim().parse().expect("数字を入力してください");
    println!("入力した数字は{}です", &input_number);
    return input_number;
}
#[test]
pub fn test_no_03() {
    assert_eq!(no_03(), 10)
}
