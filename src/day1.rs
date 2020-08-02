// enums are another object type 
#![allow(dead_code)]


enum List {
  Cons(u32, Box<List>), 
  Nil,
}

enum Status {
  Rich, 
  Poor,
}
enum Job {
  Teacher, 
  Politician,
  Programmer,
}

pub fn run() {
  println!("day1");

  // structs 
  //three main types: 
  //unit strucs
  struct Unit; 
  //fieldless and 'useful for generics' whatever that means; 
  //idk why you need this
  let _unit = Unit; 

  // touple strucst, which are named touples like Matrix previously. 
  struct Pair(f64, i32); 
  // these can be destructured 
  let p = Pair(3.0, 3); 
  let Pair(int, float) = p; 
  println!("{}, {}", int, float); 

  // 'C structs' kinda like js objects 
  struct Point {
    x: i32, 
    y: i32
  }
  let point: Point = Point {x: 24, y: 4}; 
  println!("({}, {})", point.x, point.y); 

  // destructuring and spread syntax is kinda a thing
  // uses x: 30 and y from point; 
  let new_point = Point {x: 30, ..point}; 
  println!("({}, {})", new_point.x, new_point.y); 

  use Status::{Rich, Poor};
  use Job::*; 

  let job = Programmer; 
  let status = Rich; 

  match job {
    Programmer => println!("Programmers make bank"),
    Teacher => println!("Teachers are underpaid"),
    Politician => println!("Fuck Politicians man"), 
  }
  match status {
    Rich => println!("The rich are rich ig "), 
    Poor => println!("This is unfortunate"), 
  }

  // You can make c-like enums 
  enum Color {
    Red = 0xff0000, 
    Green = 0x00ff00,
    Blue = 0x0000ff, 
    White = 0xffffff,
    Black = 0x000000,
  }

  println!("Roses are #{:06x}", Color::Red as i32); 
  println!("Violets are #{:06x}", Color::Blue as i32); 
}