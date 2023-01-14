fn main() {
  let pr = "窮鼠猫を噛む";

  for c in pr.chars() {
    print!("[{}]", c);
  }

  println!("\n文字数={}字", pr.chars().count());

  // Vec<char>に変換して処理
  let pr_chars: Vec<char> = pr.chars().collect();
  for c in pr_chars.iter() {
    print!("({})", c);
  }
  println!("\n文字数={}字", pr_chars.len());
}

