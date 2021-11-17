use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

pub fn run() {
  // there are two types of errors in Rust, recoverable and
  // unrecoverable. Recoverable come in the form of a Result<T, E>
  // While unrecoverable just call the panik! macro and stop
  // execution.

  // panic!("Message");

  // there is a panic backtrace, that will help you figure out why the program crashed

  // other type of error, recoverable returns Result<T, E>
  let f = File::open("hello.txt");
  // f is now a Result<T, E>
  let f = match f {
    Ok(file) => file,
    Err(err) => match err.kind() {
      ErrorKind::NotFound => match File::create("Hello.txt") {
        Ok(fc) => fc,
        Err(fe) => panic!("Error creating new file: {:?}", fe),
      },
      other_error => panic!("another error occured: {:?}", other_error),
    },
  };

  // this is kind of a mess tho, there are other methods that let you clean this
  // up a bit, as match statements can be a bit verbose and hard to read;

  //lets say we just want to open the file or panic. we can write
  let f = File::open("hello.txt").expect("Failed to open the file");
  // this would panic with the message "Failed to open the file" if there was an error
  // otherwise, f is now the file and no longer a Result<T, E>

  // Error propagation is returning an error from a function so the calling function
  // can better deal with it

  fn get_username(file_name: &str) -> Result<String, io::Error> {
    let f = File::open(file_name);

    let mut f = match f {
      Ok(file) => file,
      Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
      Ok(_) => Ok(s),
      Err(e) => Err(e),
    }
  }
  // but that kinda sucks, it's really common, and it's long winded
  // so there is the ? operator

  fn get_username_qm(file_name: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(file_name)?.read_to_string(&mut s)?;
    // the ? works like this: if the value is Ok(v) v will be returned from the proceeding
    // statement. if the value is Err(e), Err(e) will be returned from the whole function;
    Ok(s)
    // remember, no ; means that line is returned
  }

  // this is how you would do it normally, all of the error handling is taken care of
  // in this standard function;
  fn get_username_normal_way(file_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_name)
  }

  let user_name = get_username("Hello.txt").unwrap();
  println!("Username: {}", user_name);
}
