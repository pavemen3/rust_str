fn echo(s: &'static str) {
  println!("{}", s);
}

fn main() {
  echo("愚かな人でも黙っていると");
  echo("賢いと見られる");

  // 以下のコメント部分は失敗する
  // let s = String::from("テスト");
  // echo(&s);
}
