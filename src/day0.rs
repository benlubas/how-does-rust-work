// all the basics of variables and arrays and types 
// structs are in day1; 
use std::fmt; 

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);
// this implements Display for Matrix. allowing us 
// to use {} in a print!() and pass a Matrix and see a 2x2 
// formatted output. 
impl fmt::Display for Matrix {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {})\n({} {})", self.0, self.1, self.2, self.3) 
  }
}

fn transpose(m: Matrix) -> Matrix {
  Matrix(m.0, m.2, m.1, m.3)
}

fn array_slice(arr: &[i32]) {
  println!("Length: {}", arr.len()); 
  println!("values: {:?}", arr);
}

pub fn run() {
  // variables are immutable by default
  // type system is similar to ts. Types are inferred 
  // or explicitly defined with varName:type; 
  // semi-colons are required; 
  let fname: String = String::from("Ben"); 
  // variable nameing convention is snake_case
  let middle_i: char = 'M';
  let lname: &str = "Lubas"; 
  let weighted_gpa: f64 = 4.911; 
  // you can also annotate the type of numbers with a suffix. 
  // I personally think that's ugly as fuck; 
  let unweighted_gpa = 3.9f32;

  // two different types of string. Not entirely sure 
  // about the differences yet
  let age: i32 = 18; 
  let is_male: bool = true; 
  if is_male && unweighted_gpa > 2.5 { 
    println!("I'm {} {} {} (he/him/his) and I'm {}", fname, middle_i, lname, age);
  }
  println!("My weighted gpa was {}", weighted_gpa);

  // boolean short-circuting is fine; 
  if false && true {
    print!("NO");
  }
  // Numbers can have underscores in them to improve readability, 
  // they're ignored at compile time
  if false || 1_000_000 == 1000000 {
    println!("Yes");
  }

  //Touples 
  let touple = (1, 2, 3, true, false, "HI");
  println!("This is a touple {:?}", touple); 

  // you can destructure them 
  let (a, b, c) = ('a', 'b', 'c'); 
  println!("{} {} {}", a, b, c);

  // this is a swap. I don't really know why it works like this
  // but you can one line swap. so that's nice; 
  let one = 1; 
  let two = 2; 
  let (one, two) = (two, one);
  println!("one: {}, two: {}", one, two); 

  let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
  println!("{}", matrix); 
  println!();
  println!("{}", transpose(matrix)); 

  println!("========= Arrays ========="); 

  // arrays are static length; 
  let arr1: [i32; 5] = [1, 2, 3, 4, 5]; 
  println!("arr: {:?}", arr1);

  // filling arrays with the same value [val; len];
  let arr2: [i32; 10] = [1000; 10]; 
  println!("{:?}", arr2);

  // the & sign points the the momory address of the variable; 
  // methods can auto borrow arrays as slices; 
  array_slice(&arr1); 
  // this notation is like pythons arr[1:7]; 
  array_slice(&arr2[1 .. 7]); 
}
