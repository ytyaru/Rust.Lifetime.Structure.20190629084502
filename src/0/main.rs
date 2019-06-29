/*
 * Rustのライフタイム（構造体の定義）
 * CreatedAt: 2019-06-29
 */
#[derive(Debug)]
pub struct Text {
    part: &str, // error[E0106]: missing lifetime specifier
}
fn main() {
    let text = Text { part: "ABC" };
    println!("{:?}", text);
}

