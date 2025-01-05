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
    return sum;
}

#[test]
fn test_no_01() {
    assert_eq!(no_01(), 12345 + 23456);
}
