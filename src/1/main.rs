/*
 * Rustのライフタイム（構造体の定義）
 * CreatedAt: 2019-06-29
 */
#[derive(Debug)]
pub struct Text<'a> {
    part: &'a str,
}
fn main() {
    let text = Text { part: "ABC" };
    println!("{:?}", text);
}

