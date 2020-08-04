pub fn run() {
  // This is a string slice;
  let s: String = String::from("My name is Ben");
  let slice = &s[11..14];
  println!("Slice: {}", slice);

  // where 11 is the starting index and 14 in one more than the ending index.
  // it does work the same way as python's [s:e] syntax

  let smart_slice = &s[11..];
  println!("Smart slice: {}", smart_slice);

  let heap_string = String::from("This string");
  let stack_string = "Other string";

  fst_letter(&heap_string);
  // fst_letter(&stack_string); //error

  // both of these are fine.
  // This one is odd tho, b/c it's still very much passing String ref. and not an str ref.
  println!("{}", first_letter(&heap_string));

  //this is the 'correct' way to do it, even though they both work fine.
  println!("{}", first_letter(&heap_string[..]));
  println!("{}", first_letter(&stack_string));
}

// okay, so slices are of type str and you can take slices of Strings
// so writing functions is affected by that.

fn fst_letter(s: &String) -> &str {
  &s[..1]
}

fn first_letter(s: &str) -> &str {
  &s[..1]
}
