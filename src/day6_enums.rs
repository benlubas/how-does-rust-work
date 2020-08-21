// there are also enums.
// these are used when you want to reprecent all possible cases of something
pub fn run() {
  // for example, IP addresses can be V4 or V6;
  enum IPType {
    V4,
    V6,
  };
  // now IPType is its own type within this program.

  fn route(ip_type: IPType) {
    // we can set cases for each of the cases listed in the enumeration
    match ip_type {
      IPType::V4 => println!("V4"),
      IPType::V6 => println!("V6"),
    };
  }

  // we can also store date inside of enum variants.
  // so this can store the type and the address itself.
  enum IPAddr {
    V4(String),
    V6(String),
  };

  let localhost = IPAddr::V4(String::from("127.0.0.1"));
}
