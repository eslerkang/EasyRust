// control flow

fn main() {
  let num = 7;
  let num2 = 10;
  if num == 7 && num2 == 10{
    println!("They both match!!");
  } else if num == 6 { // && = and, || = or
    println!("It's six");
  } else {
    println!("What are you!??");
  }

  // expression based language
  let my_num = 5u8;
  let second_num = match my_num { // better than switch
    0 => 23,
    1 => 234,
    _ => 0,
  };
  println!("second is {}", second_num);
}