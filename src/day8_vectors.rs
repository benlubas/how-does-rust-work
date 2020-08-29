#![allow(dead_code)]
pub fn run() {
  // vectors are like array lists 
  // Vec<T> is standard, so it's namespaced in
  let mut v: Vec<i32> = Vec::new(); 
  // there is also a vec! macro that creates vectors and infers type. 
  let v2 = vec![1, 2, 3]; 

  v.push(5); 
  v.push(0); 
  v.push(2); 

  // two ways to get a value from a vec, &[index] or get(index)
  println!("i 0: {}", &v[0]); // this returns the value or crashes on out of bounds
  println!("i 1: {:?}", v.get(1)); // this returns Option<&T>

  match v.get(20) {
    Some(&x) => println!("{}", x), 
    None => println!("This index is out of bounds") 
  } 

  // looping through each immutable value
  for i in &v {
    println!("{}", i); 
  }

  // looping through with mutable values
  // add 30 to each value
  for i in &mut v {
    *i += 30; // the * is called the dereference opporator. 
              // it pretty much says, follow this pointer and get the value. 
  }

  println!("{:?}", v); 

  // storing multiple types in one vec is done with enums. 
  #[derive(Debug)]
  enum Prop {
    Int(i32), 
    Text(String), 
    Float(f64),
  }
  let mut mixed: Vec<Prop> = Vec::new(); 

  mixed.push(Prop::Int(40));
  mixed.push(Prop::Text(String::from("hello")));
  mixed.push(Prop::Float(3.1415926));

  println!("{:?}", mixed);

  // other Vec<T> methods
  let length: usize = v.len(); 

  let last: Option<i32> = v.pop(); 

  let mut v2 = vec![56, 57, 58]; 
  v.append(&mut v2); 

  v2.clear(); 

  let index_2: i32 = v.remove(2); 

  // and some more as well

}