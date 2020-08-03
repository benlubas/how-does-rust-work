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
}
