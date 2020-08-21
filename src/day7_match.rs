pub fn run() {
  // This is a match statement. It is kinda like a switch statement
  // but it requires that you cover all possible cases.
  #[derive(Debug)]
  enum State {
    WA,
    CA,
    AZ,
    PA,
    MA,
  }
  enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
  }
  // on another note, the -> vs => is really annoying, and should not be a thing,
  // it should all be =>, But I understand that they are different. So, whatever. I'll
  // get used to it.
  fn value_of_coin(coin: Coin) -> i32 {
    match coin {
      Coin::Penny => 1,
      Coin::Nickel => 5,
      Coin::Dime => 10,
      Coin::Quarter(s) => {
        // herer we can extract values from Enums that store values.
        println!("State Quarter From {:?}", s);
        25
      }
    }
  }

  // I'm also going to throw Option<T> into this file.
  // Option<T> is like an optional value. Sota, it can be None or Some(value);
  // this is essentially Rust's Null value, as there isn't actually one.
  // notice that Option<T>::None and Option<T>::Some() are namespaced in.

  let maybe_none: Option<i32> = Some(30);
  let maybe_none: Option<i32> = None;

  let mut starts_as_null: Option<i64> = None;
  // throws error, needs to be Some(39);
  // starts_as_null = 39;
  starts_as_null = Some(39);

  // you can match with Option<T>'s
  fn plus_plus(x: Option<i32>) -> Option<i32> {
    match x {
      Some(x) => Some(x + 1),
      None => None,
    }
  }

  // there is a 'default' case in a match statement.
  let n: i32 = 3;
  println!("{}", n);
  let n: String = match n {
    1 => String::from("one"),
    2 => String::from("two"),
    3 => String::from("three"),
    4 => String::from("four"),
    // we don't want to write out every single possible number here.
    _ => String::from("Not one, two, three, or four"),
  };
  println!("{}", n);

  // let say you have one thing that you want to check for. Say I want to know if b is 4;
  let b = 4;
  match b {
    4 => println!("Found 4!"),
    _ => (), // do nothing.
  }

  // that's a lot of code.
  // we can write it as this instead...
  let b = 5;
  if let 4 = b {
    println!("Found 4!"); // as a side note, these ; seem to be optional
  } else {
    println!("Not 4"); // <---
  }
}
