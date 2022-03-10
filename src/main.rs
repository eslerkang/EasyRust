fn main() {
  let mut counter = 0;
  let mut counter2 = 0;
  'first: loop {
    counter += 1;
    println!("the counter is: {}", counter);
    if counter > 9 {
      println!("now entering the second loop");

      'second: loop {
        println!("second counter is: {}", counter2);
        counter2 += 1;
        if counter2 == 3 {
          break 'first;
        }
      }
    }
  }

  counter = 0;
  while counter != 5 {
    counter += 1;
    println!("{}", counter);
  }

  for _number in 0..=3 {
    // 0..3 => 0 1 2
    // 0..=3 => 0 1 2 3
    println!("I don't need number");
  }

  // break can have return value
  counter = 5;

  let number = loop {
    counter += 1;
    if counter %53 == 3 {
      break counter;
    }
  };
  println!("number is now: {}", number);
}