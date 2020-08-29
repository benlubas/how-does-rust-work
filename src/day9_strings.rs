pub fn run() {
  // there are a bunch of different types of strings in Rust,
  // most used are String and str;
  let s: &str = "This is a string";
  // there are two ways to make a String,
  // there is no diference between the two
  let _string: String = String::from(s);
  let mut string: String = s.to_string();

  println!("{}", string);

  // you can push things to the string like this
  // push str
  string.push_str(", and this was added");
  // push char
  string.push('!');

  println!("{}", string);

  // concat
  let first_name = String::from("Ben ");
  let last_name = String::from("Lubas");
  let full_name = first_name + &last_name;
  // note that first_name goes out of scope here while last_name does not
  // error:
  // println!("First name: {}", first_name);
  // no error
  println!("Last name: {}", last_name);
  println!("Hi, I'm {}", full_name);

  let one = String::from("tic");
  let two = String::from("tac");
  let three = String::from("toe");

  //for longer concatinations it's sometimes better to use format!
  let tic_tac_toe = format!("{}-{}-{}", one, two, three);
  println!("{}", tic_tac_toe);
  // this doesn't take ownership of any of those variables
  println!("{}-{}-{}", one, two, three);

  let state = "Pennsylvania";
  for l in state.chars() {
    print!("{} ", l);
  }
  println!();

  let first_letter: Option<char> = state.chars().nth(0);
  println!("{:?}", first_letter);

  match first_letter {
    Some(letter) => println!("There is a letter here, it is: {}", letter),
    None => println!("Nothing here"),
  }
}
