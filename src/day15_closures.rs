pub fn run() {
  println!("Clotures "); 


  // clotures are like anonymous functions with extra steps lol. 
  // They don't behave exactly like other functions in rust. 

  let dist = |x: i32, y: i32, i: i32, j: i32| (((x - i).pow(2) + (y - j).pow(2)) as f64).sqrt(); 
  let d = dist(0, 0, 0, 5); 
  println!("d: {}", d); 
  
  
  // notably, they don't _need_ type annoytation, and they can access variables that are 
  // in the scope they're defined in. 

  let y = 2; 
  let cloture = |x| x + y; 

  assert_eq!(cloture(7), 9); 

  println!("y: {}", y); 

  // There is a keyword (move) that lets you move any used variables into the 
  // closure. So you can't access them after 
  //this doesn't happen with stack allocated stuff, b/c it just get's coppied 

  let cloture_m = move |x| x + y; 
  println!("y: {}", y); 
  assert_eq!(cloture_m(7), 9); 

  let a = vec![1, 2, 3]; 
  let push_to_vec = move |b| {
    let mut n = a.clone();
    n.extend(b);
    n
  }; 

  // now a is moved, this doesn't work 
  // println!("{:?}", a); 

  println!("{:?}", push_to_vec(vec![1, 2, 3])); 
  
  // these things sees a little odd, and I'm not sure that I like how they work
  // this language in general is really strict, and these things sure do show that. 
}