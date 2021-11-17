pub fn run() { 
  println!("Hello from Generics ");


  // generics allow us to write code that can opperate over different types 
  // of data while not necessarily knowing the datatype at complite time 

  // say we want to find the largest item in a list of numbers and/or a list
  // of chars. Right now we'd need two different functions: 
  fn largest_i32 (list: &[i32]) -> &i32 {
    let mut largest = &list[0]; 
    for i in list {
      if i > largest {
        largest = i;
      }
    }
    largest 
  }
  fn largest_char (list: &[char]) -> &char {
    let mut largest = &list[0]; 
    for i in list {
      if i > largest {
        largest = i; 
      }
    }
    largest 
  }

  let l1 = vec![1, 2, 54, 3, 12, 31, 192, 32]; 
  println!("largest num in l1 is {}", largest_i32(&l1)); 
  let l2 = vec!['c', 'a', 'd', 'u', 'b']; 
  println!("largest char in l2 is {}", largest_char(&l2)); 


  // notice how the code in those functions is really repetitive. 
  // in fundies we would abstract away the differences. But here the only differences 
  // have to do with type. So this is where generics come in. 

  // our new largest function becomes: 

  fn largest<T: PartialOrd>(list: &[T]) -> &T { 
    let mut largest = &list[0]; 
    for i in list {
      if i > largest {
        largest = i; 
      }
    }
    largest 
  }
  println!("largest num: {}", largest(&l1)); 
  println!("largest char: {}", largest(&l2)); 

  // but the > is giving us problems now. This is b/c the generic type T might not 
  // implement the > operator. For example, a String > String comparison does not work. 

  // to fix this, we use traits. Traits are really long and confusing. But it's just 
  // something that a datatype implements. The PartialOrd trait is the trait of comparision 
  // via < or >, and is part of the standard library. 
  
  // Let's look at structs real quick. Those can also take advantage of generics 
  struct Point<T> {
    x: T, 
    y: T,
  }

  let integer_point: Point<i32> = Point {x: 1, y: 3}; 
  let _float_point: Point<f64> = Point {x: 1.4, y: 3.9}; 
  // the types don't have to be explicit here, they can be inferred. 

  // let nope = Point {x: 1, y: 1.2}; This won't work because T has to be one type,
  // it can't be both 


  // to get around this, just use two generics. 
  struct MultPoint<T, U> {
    x: T, 
    y: U,
  }

  let _yup = MultPoint {x: 1, y: 1.2}; 
  // T and U can both be the same thing. 
  let _also_yep = MultPoint {x:1, y:1}; 

  // we can implement functions on structs with generics too 
  impl<T> Point<T> {
    fn x(&self) -> &T {
      &self.x
    }
  }
  println!("Point.x = {}", integer_point.x);

  // using generics has no performance cost on your program. 

}
