// comment in rust

fn main() {
  println!("Hello World!");
  println!("I am a Rustacean!");

  let x = 5 + 4;
  let y = 7;
  let z = "hello!";
  println!("{} - Is `x` 10 or 100? x={}; y={}",z,x,y);
  // println! is a macro
  // macro is a way for copying code and it aware of the context that the code is running
  let b = 255;
  println!("Base 10: {}",b);
  println!("Base 2 (binary): {:b}",b);
  println!("Base 8 (octal): {:o}",b);
  println!("Base 16 (hexadecimal): {:x}",b);
  println!("Base 16 (hexadecimal): {:X}",b);
  // right-justify text and add _ to the left
  println!("{k:_>25}",k=b);
  // left-justify text and add _ to the right
  println!("{k:_<25}",k=z); // k is not a variable!
  println!("{txt:->25}",txt="si"); // txt is not a variable!
  println!("My name is {0}, {1} {0}", "James", "Bond");
  println!("{k:_<width$}", k="li", width=x);
  println!("{k:_>width$}", k="li", width=y);

  #[allow(dead_code)] #[derive(Debug)] struct Structure(i32);
  // let number: f64 = 1.0;
  // let width: usize = 5;
  println!("This struct `{:?}` won't print ...", Structure(3));

  let num = 113.141592;
  println!("{num} is roughly {0:5.2}",num);
  println!("{num} is roughly {0:.2}",num);
  // 112 is way bigger than the total number after `.` -> maybe it pick something from mem
  // and it looks like it is printing random number
  println!("{num} is roughly {0:.112}; print some random number !!!",num);
}

// compile: rustc ./no_module_code/1_helloworld.rs -o ./bin/1_helloworld