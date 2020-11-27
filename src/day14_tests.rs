fn add1(x: i32) -> i32 {
  x + 1
}
fn first<T>(list: &[T]) -> &T {
  &list[0]
}
fn last<T>(list: &[T]) -> &T {
  &list[list.len() - 1]
}
fn rest<T>(list: &[T]) -> &[T] {
  if list.len() == 0 {
    panic!("Given empty list"); 
  }
  &list[1..]
}

pub fn run() {
  println!("From tests"); 
}

// we label tests with #[test] 
// tests run when we call cargo run. 
// be default they run in parallel on threads, so don't make them depend on one another. 
#[test] 
fn test_add1() {
  assert_eq!(add1(1), 2);
  assert_eq!(add1(3), 4);
  assert_eq!(add1(-2), -1);
}
#[test]
fn test_rest () {
  assert_eq!(rest(&vec![1, 2, 3])[0..], [2, 3]);  
  assert_eq!(rest(&vec!['f', '3', 'a', 'h']), ['3', 'a', 'h']); 
}
// the #[should_panic] flag can be added to make sure something panics when it should 
// this test panics because it tries to take a slice starting at one of an empty list. 
#[test]
#[should_panic]
fn rest_panic() {
  let v = vec![0]; 
  rest(rest(&v));
}

// By default anything printed to std::out will only print if the test 
// fails. 

// cargo test -- --show-output 
// to see the printed output from a test. 

//