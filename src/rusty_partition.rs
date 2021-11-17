fn main() {
  let v1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]; 
  let v2 = vec![2, 4, 6]; 
  let v3 = vec![1, 3, 5]; 
  let v4 = vec![-43, -12, 0, 124894235, 234, 243, 0, -12, -14];
  let r1 = evens_and_odds(v1);
  let r2 = evens_and_odds(v2);
  let r3 = evens_and_odds(v3);
  let r4 = evens_and_odds(v4);
  assert_eq!(r1, (vec![0, 2, 4, 6, 8], vec![1, 3, 5, 7, 9]));
  assert_eq!(r2, (vec![2, 4, 6], vec![]));
  assert_eq!(r3, (vec![], vec![1, 3, 5]));
  assert_eq!(r4, (vec![-14, -12, -12, 0, 0, 234], vec![-43, 243, 124894235]));
}

// numbers: a vector input of integers
// returns: (evens, odds), a touple of Vec<i32>'s with all the evens and odds respectively, in 
// sorted order
fn evens_and_odds(numbers: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let mut evens: Vec<i32> = Vec::new(); 
    let mut odds: Vec<i32> = Vec::new(); 

    for n in numbers {
        if n.abs() %2 == 0 {
            evens.push(n); 
        } else {
            odds.push(n); 
        }
    }

    evens.sort(); 
    odds.sort(); 

    (evens, odds)
}
