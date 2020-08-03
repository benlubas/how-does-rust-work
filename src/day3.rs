// ownership & memory

pub fn run() {
  // these are stack allocated and can't be mutated.
  let s_literal = "This is a string literal";
  // This is a String, it can be mutated, and is heap
  // allocated
  let mut s = String::from(s_literal);
  println!("{}", s);
  s.push_str(". Actually, it's not");
  println!("{}", s);
}
