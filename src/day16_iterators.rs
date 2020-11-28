pub fn run() {
  // iterators are just something that can be iterated over. 
  // so like a vec![1, 2, 3, 4].iter()

  let v = vec![1, 2, 3, 4]; 
  let v_iter = v.iter(); 

  let mut sum = 0; 
  for n in v_iter {
    sum += n;
  }
  assert_eq!(sum, 10);
}
