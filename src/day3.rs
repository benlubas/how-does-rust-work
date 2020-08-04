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

  // this is a String, the variable s1 stores the pointer to the
  // heap allocated memory that stores the contents of the string
  let s1 = String::from("This is a string");
  // In other languages, this "shallow copies" the string to s2 by
  // simply copying the pointer from s1 to s2, so changing s1 also
  // changes s2. But b/c memory has to be deallocated in rust and is
  // done so automatically, the value stored at s1 is now owned by s2,
  // and thus, s1 is no longer useable.
  let s2 = s1;
  // this line throws an error
  // println!("{}", s1);
  println!("{}", s2);

  // if you want two variables that are equal, you use clone()
  let s3 = s2.clone();
  println!("s1: {}, s2: {}", s2, s3);

  // this is only the behavior for heap allocated variables
  let x = 2;
  let y = x;
  // this is still valid, x does not loose ownership of 5, as these
  // vaules are stored on the stack, and aren't expensive to copy.
  println!("x: {}, y: {}", x, y);

  // these are said to be of type Copy, in that they are auto coppied when
  // assignments like the above (31-32) happen.

  // touples that only contain copy values are copy,
  let copy_touple = (1, 2, 3);
  // touples that contain non-copy values are non-copy
  let non_copy_touple = (1, 2, String::from("string"));
  let c = copy_touple;
  println!("{:?}, {:?}", copy_touple, c); // legal
  let c = non_copy_touple;
  // println!("{:?}, {:?}", non_copy_touple, c); // illegal

  // functions can take ownership of a variable or make a copy;
  let s1 = String::from("This is a string");
  let x1 = 4;
  takes_ownership(s1);
  makes_copy(x);
  // this one throws an error.
  // println!("trying to use s: {}", s1);
  println!("trying to use x: {}", x1);

  // you can get ownership back if a variable is returned.
  let s2 = String::from("This is another string");
  let s2 = returns_ownership(s2);
  println!("Using s2: {}", s2);

  // but this is annoying, so we use references.
  let s3 = String::from("hello");
  println!("len: {}", get_len(&s3));

  // when you want to alter something you're borrowing, you have to
  // specify that, a alot.
  let mut q = String::from("What is the meaning of life");
  add_question_mark(&mut q);
  println!("q: {}", q);
  // There can only be one mutable ref to a varable in each scope...
  // but this is fine;
  add_question_mark(&mut q);
  println!("q: {}", q);
}

fn get_len(s: &String) -> usize {
  s.len()
}

fn add_question_mark(s: &mut String) {
  s.push_str("?");
}

fn takes_ownership(s: String) {
  println!("s: {}", s);
}

fn makes_copy(x: i32) {
  println!("x: {}", x);
}

fn returns_ownership(s: String) -> String {
  s
}
