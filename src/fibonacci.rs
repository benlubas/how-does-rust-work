pub fn run() {
  // println!("What number of the sequence do you want? ");
  // let input = loop {
  //   let mut input = String::new();
  //   std::io::stdin()
  //     .read_line(&mut input)
  //     .expect("failed to read line");

  //   match input.trim().parse() {
  //     Ok(num) => break num,
  //     Err(_) => println!("We need a number"),
  //   };
  // };

  for i in 0..50 {
    println!("{}", get_nth_num(i));
  }
}

fn get_nth_num(n: u64) -> u64 {
  let mut p1: u64;
  let mut p2: u64 = 0;
  let mut current: u64 = 1;
  for _ in 0..n {
    p1 = p2;
    p2 = current;
    current = p1 + p2;
  }
  current
}
