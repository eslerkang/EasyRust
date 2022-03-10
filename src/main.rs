#[derive(Debug)]
struct Animal {
  age: u8,
  animal_type: AnimalType
}

#[derive(Debug)]
enum AnimalType {
  Cat,
  Dog
}

impl AnimalType {
  fn check_type(&self) {
    use AnimalType::*;

    match self {
      Cat => println!("a cat..?!"),
      Dog => println!("a dog..?!")
    }
  }
}

// implement
// impl 여러개 가능

impl Animal {
  fn new_old_cat() -> Self {
    Self {
      age: 16,
      animal_type: AnimalType::Cat
    }
  }
}

impl Animal {
  fn new(age: u8, animal_type: AnimalType) -> Self { // Self = Animal / Also can write Animal
    Self {
      age, animal_type
    }
  }

  fn new_cat(age: u8) -> Self {
    Self {
      age,
      animal_type: AnimalType::Cat
    }
  }

  fn new_dog(age: u8) -> Self {
    Self {
      age,
      animal_type: AnimalType::Dog
    }
  }

  fn print(&self) {
    println!("I'm an {:#?}", self);
  }

  fn change_to_cat(&mut self) {
    self.animal_type = AnimalType::Cat;
  }
  
  fn change_to_dog(&mut self) {
    self.animal_type = AnimalType::Dog;
  }
  
  fn check_type(&self) {
    use AnimalType::*;
    match self.animal_type {
      Cat => println!("Wow what a cat!"),
      Dog => println!("Wow what a dog!")
    }
  }
}

fn main() {
  let animal = Animal::new(10, AnimalType::Dog);
  // associated function 
  println!("{:#?}", animal);

  let mut dog = Animal::new_dog(8);
  dog.print(); // dot operator
  // syntactic sugar
  
  // same as
  Animal::print(&dog);

  dog.change_to_cat();
  dog.print();
  dog.change_to_dog();
  dog.print();


  use AnimalType::*;
  let cat = Animal::new(1, Cat);
  let dog = Animal::new(2, Dog);
  cat.animal_type.check_type();
}