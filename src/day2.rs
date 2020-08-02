// there is some stuff about functions in here.
// there is also more info about if else statements
// and loops

// Firstly, exporting a function so that it can be
// called in another file is done with the pub (for public)
// keyword
pub fn run() {
  a_test_function(4);
  println!("{}", returning(false));

  // some conditional things
  // let num: i32 = 6;
  //this doesn't work
  // if num { }

  // else if chains.
  if false {
    //do this
  } else if 5 == 8 - 3 {
    //do this
  } else {
    //do this
  }

  // conditional assignment
  // unfortunately the {} are required;
  let html = if true { "<br/>" } else { "<hr/>" };
  // both have to return the same type.
  // can't have one return "hi" and the other 47
  println!("{}", html);

  //LOOPS
  // the loop keyword will excecute a loop untill it
  // hits the break keyword;
  let mut i = 0;
  loop {
    i += 1;
    if i == 10 {
      break;
    } else {
      println!("{}", i);
    }
  }

  // you can also return values from a loop.
  i = 0;
  let result = loop {
    i += 1;
    if i == 3 {
      break i * 39;
    }
  };
  println!("{}", result);

  // swap lines below for this to work
  // let mut input = String::new();
  let mut input = String::from("exit");
  //while loops work the same way
  while input != "exit" {
    println!("Type:");
    input = String::from("");
    match std::io::stdin().read_line(&mut input) {
      Ok(_) => input = String::from(input.to_owned().trim()),
      Err(e) => println!("ERR: {}", e),
    };
  }

  let arr = [1, 2, 3, 4, 5];
  let mut i = 0;
  while i < arr.len() {
    print!("{} ", arr[i]);
    i += 1;
  }
  println!();
  //or
  for n in arr.iter() {
    print!("{} ", n);
  }
  println!();
  // there are also ranges in Rust!
  // 1..4 is line range(1, 4) in python;
  for i in 1..4 {
    print!("{} ", i);
  }
}

// parameters must be declared with a type
fn a_test_function(x: i32) {
  print!("x: {}", x);
}

// you can return early with the return keyword
// otherwise, the last line without a semicolon
// will be returned.
fn returning(is_even: bool) -> i32 {
  // if statements don't need ( )
  // multi line if's need { }
  if is_even {
    return 6;
  }
  5
}
