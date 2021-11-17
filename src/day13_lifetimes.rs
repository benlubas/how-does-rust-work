pub fn run() {
  // Lifetimes are almost always inffered by the compiler, and don't need to be annotated 
  // but sometimes multiple lifetimes make sense, so we have to step in. 

  // A lifetime is how long a variable can be referenced (kinda)

  // take the following for example: 

  // let r; 
  // {
  //   let x = 4; 
  //   r = &x; 
  // }
  // println!("{}", r); 

  // this doesn't work, b/c x goes out of scope, and gets dealocated, but r is still 
  // pointing to it. Rust prevents us from using r b/c of this. 

  // now with functions. 
  let s1 = "this string"; 
  let s2 = String::from("another string");
  let l = longest(s1, s2.as_str()); 
  println!("{}", l); 
  //without the <'a> generic lifetime, Rust doesn't know if the lifetime of the thing 
  // we're returning should be linked to the lifetime of x or y. So we tell it 
  // that x and y have the same lifetime. I'm not sure that it's actually like that
  // but that's my best guess at this. It's really confusing. 

  fn longest<'a> (x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
      x
    } else {
      y
    }
  }

  // it's important to note that we can't actually change the lifetimes of variables 
  // by annotating them. We just let the compiler know what to enforce reguarding lifetimes 

  // The result of the annotation above is that the result of calling longest will live 
  // only as long as both params live. If one goes out of scope, so does the result of 
  // the function call

  // there are some rules that the rust compiler follows to ease the requirement of 
  // annotating lifetimes. But those are pretty long and I just don't feel like reading 
  // throguh them today. 
}