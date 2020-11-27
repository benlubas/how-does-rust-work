pub fn run() {
  fn sum(x: i32, y: i32) -> i32 {
    x + y
  }
  fn min_to_sec(min: i32) -> i32 {
    min * 60
  }
  fn triangle_area(base: f64, height: f64) -> f64 {
    base * height * 0.5
  }
  fn remainder(big: i32, small: i32) -> i32 {
    // it looks like this should always return 0, but integer divison floors it.
    big - big / small * small
  }
  fn absolute_sum(arr: &[i32]) -> i32 {
    let mut total = 0;
    for i in arr {
      total += i.abs();
    }
    total
  }
  // return true if last dig of a * last dig of b = last dig of c
  fn last_dig(a: i32, b: i32, c: i32) -> bool {
    // there has to be a cleaner way to do this.
    let a = format!("{}", a).chars().last().unwrap();
    let b = format!("{}", b).chars().last().unwrap();
    let c = format!("{}", c).chars().last().unwrap();
    let a = a.to_string().parse::<i32>().unwrap();
    let b = b.to_string().parse::<i32>().unwrap();
    let c = c.to_string().parse::<i32>().unwrap();
    a * b == c
  }
  fn no_odds(arr: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();
    for n in arr {
      if n % 2 == 0 {
        output.push(*n);
      }
    }
    output
  }
  fn seeing_double(s: &String) -> String {
    let mut doubled = String::new();
    for i in s.chars() {
      doubled.push(i);
      doubled.push(i);
    }
    doubled
  }
  fn alphebetize_word(word: &String) -> String {
    let mut vec = word.split("").collect::<Vec<&str>>();
    vec.sort(); 
    String::from(vec.join("").trim())
  }
  println!("{}", alphebetize_word(&String::from("hello there buddy")));

}
