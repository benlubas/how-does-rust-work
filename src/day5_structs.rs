pub fn run() {
  #![allow(dead_code)]
  // a struct is like an object
  struct Player {
    name: String,
    rank: i32,
  };

  let p1 = Player {
    name: String::from("Ben"),
    rank: 1,
  };

  println!("{} is rank {}", p1.name, p1.rank);

  let mut p2 = Player {
    name: String::from("Jake"),
    rank: 2,
  };
  p2.name = String::from("Jaquee");

  struct User {
    uid: i32,
    username: String,
    email: String,
    active: bool,
  }
  fn init_user(username: String, email: String, uid: i32) -> User {
    // notice how variable assignment shorthand exists like in svelte.
    // same varibale name as the struct variable name, just write it and it's
    // auto assigned.
    User {
      uid,
      username,
      email,
      active: true,
    }
  }
  let mut next_id = 0;

  let new_user = init_user(
    String::from("_TwoNine"),
    String::from("lubasking29@gmail.com"),
    next_id,
  );
  next_id += 1;

  println!(
    "New User: \nUsername: {}\nEmail: {}\nActive: {}",
    new_user.username, new_user.email, new_user.active
  );

  // lets say we want a new user with the same email but a different username,
  let user2 = User {
    username: String::from("ThreeNine"),
    ..new_user
  };
  //this will auto fill the rest of the values from new_user. (note: this doesn't work exactly like javascript spread syntax)

  // this is a touple struct.
  // adding the debug thing lets you print it with a {:?}.
  #[derive(Debug)]
  struct Point(i32, i32);
  let origin = Point(0, 0);

  println!("x: {}, y: {}", origin.0, origin.1);
  println!("{:?}", origin);

  // we can implent functions to be called off of structs
  struct Rectangle {
    width: i32,
    height: i32,
  };
  // we can have multiple impl blocks per struct (it's just not useful here).
  impl Rectangle {
    //these two are methods b/c they take self as a param
    fn area(&self) -> i32 {
      self.width * self.height
    }
    fn can_hold(&self, r: &Rectangle) -> bool {
      self.width > r.width && self.height > r.height
    }
    //this is an associated function b/c it doesn't take self as a param.
    // String::from is an associated function
    fn square(size: i32) -> Rectangle {
      Rectangle {
        width: size,
        height: size,
      }
    }
  };
  let rect = Rectangle {
    width: 10,
    height: 20,
  };
  let rect2 = Rectangle {
    width: 200,
    height: 5,
  };
  let rect3 = Rectangle {
    width: 20,
    height: 100,
  };
  let area = rect.area();
  println!("Area: {} sq units", area);

  // fun if statements inside of functions. This is so weird to me.
  println!(
    "one {} hold two",
    if rect.can_hold(&rect2) {
      "can"
    } else {
      "cannot"
    }
  );
  println!("two can hold three ? {}", rect2.can_hold(&rect3));
  println!("three can hold one ? {}", rect3.can_hold(&rect));
  let sq = Rectangle::square(5);
  println!("The area of the square is {}", sq.area());
}
