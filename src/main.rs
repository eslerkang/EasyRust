fn match_num(input: i32) {
  match input {
    number @ 0..=10 => println!("it's between 0 10. it's the number {}", number),
    _ => println!("Haam")
  };
}

fn main() {
  match_num(10);
  match_num(100);
}