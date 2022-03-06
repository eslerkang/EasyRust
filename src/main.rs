// control flow

fn main() {
  let sky = "cloudy";
  let temperature = "warm";

  // match statemets is like filter -> order is important
  match (sky, temperature) {
    ("cloudy", "cold") => println!("not very nice"),
    ("clear", "warn") => println!("nice!"),
    ("cloudy", _) => println!("just cloudy and something"),
    _ => println!("Not sure...")
  };

  let children = 5;
  let married = true;
  match (children, married) {
    (c, m) if !m => println!("{}", c),
    (c, m) if c == 0 && m => println!("married but no child"),
    _ => println!("IDK")
  }
}