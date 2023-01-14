fn main() {
  // 文字列リテラルは &str 型
  let ss: &str = "気前良く与えるなら豊かになる";
  // &strからStringへの変換
  let so1: String = String::from(ss);
  let so2: String = ss.to_string();
  // String から &strへの変換
  let ss2: &str = &so1;
  let ss3: &str = so1.as_str();
  // 画面に表示
  println!("{}\n{}\n{}\n{}", so1, so2, ss2, ss3);
  // 参照型のポインターアドレスを表示
  println!("{:p}\n{:p}", ss2, ss3);
}

