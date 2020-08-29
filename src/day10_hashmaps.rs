use std::collections::HashMap;

pub fn run() {
  // hashmaps are like dictionaries in python they store key value pairs
  // as you can see, they're not in the namespace by default

  let mut scores = HashMap::new();

  let key = String::from("Blue");
  let value = 10;

  scores.insert(key, value);
  scores.insert(String::from("Green"), 20);

  let my_team = String::from("Blue");
  let my_score = scores.get(&my_team);
  // returns Option<T> as with all get methods
  println!("{} has {:?} points", my_team, my_score);

  for (k, v) in &scores {
    println!("{} - {}", k, v);
  }

  // we can override values
  scores.insert(String::from("Blue"), 11);
  println!("{:?}", scores.get("Blue"));
  // the new value for key "Blue" is 11

  // here we are checking if the values exist and then inserting them only if
  // they're not found. So Yellow - 0 is added and Blue - 0 is not
  scores.entry(String::from("Yellow")).or_insert(0);
  scores.entry(String::from("Blue")).or_insert(0);

  println!("{:?}", scores);

  let text = "this is a sentence and some other words appear in this sentence";
  let mut word_counts = HashMap::new();
  for word in text.split_whitespace() {
    // this is some confusing code.
    // the count variable is a reference to the value (either existing or newly 0)
    let count = word_counts.entry(word).or_insert(0);
    // the * dereferences, so it says 'follow this pointer' and then performs the addition
    *count += 1;
    // very cool.
  }
  println!("{:?}", word_counts);
}
