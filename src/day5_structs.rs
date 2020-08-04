#![allow(dead_code)]
pub fn run() {
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
  struct Point(i32, i32);
  let origin = Point(0, 0);

  println!("x: {}, y: {}", origin.0, origin.1);
}
