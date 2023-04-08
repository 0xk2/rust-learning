#[derive(Debug)] struct Structure(i32);
#[derive(Debug)] struct Deep(Structure);

#[derive(Debug)] struct Person<'a> {
  name: &'a str,
  age: u8
}


fn main() {
  // printing with {:?} is similar to with `{}`
  println!("{:?} months in a year",12);
  println!("{1:#?} {0:?} is the {actor:?} name",
          "Slater",
          "Christian",
          actor="actor's");
  let x = Structure(8);
  println!("Now {:?} will print!", Structure(39));
  println!("New {:?} will print!", Deep(x));

  let a_name = "Peter";
  let an_age = 27;
  let peter = Person{ name: a_name, age: an_age };
  println!("{:#?}", peter);
  println!("{:?}", peter);
  println!("name : {0}; age: {1}", peter.name, peter.age);
}