// struct = and
// enum = or

// we start it's name Capital and use Cammel case
enum ThingsInTheSky {
  Sun, // 0
  Stars, // 1
  // Other
}

enum Mood {
  Happy,
  Sleepy,
  NotBad,
  Angry
}

#[derive(Debug)]
enum Season {
  Spring,
  Summer,
  Autumn,
  Winter
}

enum Star {
  BrownDwarf = 10,
  RedDwarf = 50,
  YellowStar = 100,
  RedGiant = 1000,
  DeadStar
}

enum Number {
  U32(u32),
  I32(i32),
}

fn takes_number(input: i32) -> Number {
  match input.is_positive() {
    true => Number::U32(input as u32),
    false => Number::I32(input)
  }
}


fn match_mood(mood: &Mood) -> i32 {
  use Mood::*;
  
  match mood {
    Happy => 10,
    Sleepy => 6,
    NotBad => 7,
    Angry => 2
  }
}

fn create_skystates(time: i32) -> ThingsInTheSky {
  match time{
    6..=18 => ThingsInTheSky::Sun,
    _ => ThingsInTheSky::Stars
  }
}

fn check_skystates(state: ThingsInTheSky) {
  match state {
    ThingsInTheSky::Sun => println!("I can see the sun"),
    ThingsInTheSky::Stars => println!("I can see the stars")
  };
}

fn main() {
  use Star::*;
  use Season::*;
  let starvec = vec![BrownDwarf, RedDwarf, YellowStar, RedGiant, DeadStar];
  for star in starvec {
    match star as u32 {
      size if size <= 80 => println!("not the biggest star: {}", size),
      size if size >= 80 => println!("pretty big star: {}", size),
      _ => println!("some other star")
    }
  }
  let four_seasons = vec![Spring, Summer, Autumn, Winter];
  for season in four_seasons {
    println!("the number is: {:?}", season);
  }
  let time = 8;
  let sky_state = create_skystates(time);
  check_skystates(sky_state);
  // check_skystates(create_skystates(10));
  let m = Mood::NotBad;
  let happiness = match_mood(&m);
  println!("{}", happiness);

  let my_vec = vec![takes_number(-100), takes_number(8)];
  for item in my_vec {
    match item {
      Number::I32(number) => println!("its i32 with the value {}", number),
      Number::U32(number) => println!("its u32 with the value {}", number)
    }
  }
}