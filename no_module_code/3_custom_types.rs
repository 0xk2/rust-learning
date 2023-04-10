#![allow(dead_code)]

// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad ! PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.

enum WebEvent {
  // An `enum` variant may be either be `unit-like`,
  PageLoad,
  PageUnload,
  // like tuple structs,
  KeyPress(char),
  Paste(String),
  // or c-like structures.
  Click {x: i64, y: i64},
}

// A function which takes a `WebEvent` enum as an argument and returns nothing.
fn inspect(event: WebEvent) {
  match event {
    WebEvent::PageLoad => println!("page loaded"),
    WebEvent:: PageUnload => println!("page unloaded"),
    // Destrcuture `c` from inside the `enum` variant.
    WebEvent::KeyPress(c) => println!("pressed `{}`.",c),
    WebEvent::Paste(s) => println!("pasted \"{}\"", s),
    // Destrucuture `Click` into `x` and `y`
    WebEvent::Click{x,y} => {
      println!("clicked at x={}, y={}", x,y);
    }
  }
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
  Add,
  Substract
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
  fn run(&self, x: i32, y: i32) -> i32 {
    match self {
      Self::Add => x+y, // Self is an alias
      Self::Substract => x-y,
    }
  }
}
// alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

enum Status {
  Rich, 
  Poor,
}

enum Work {
  Civilian,
  Soldier,
}

static LANGUAGE: &str = "Rust";
const THRESHOLD:i32 = 10;

fn is_big(n: i32) -> bool {
  n > THRESHOLD
}

fn main() {
  let pressed = WebEvent::KeyPress('x');
  // `to_owned()` creates na owned `String` from a string slice
  let pasted = WebEvent::Paste("my text".to_owned());
  let click = WebEvent::Click{x: 20, y: 80};
  let load = WebEvent::PageLoad;
  let unload = WebEvent::PageUnload;

  inspect(pressed);
  inspect(pasted);
  inspect(click);
  inspect(load);
  inspect(unload);

  let x = Operations::Add;
  println!("{}",x.run(1 , 2));

  use crate::Status::{Poor, Rich};
  use crate::Work::*;

  let status = Rich;
  let work = Soldier;

  match status {
    Rich => println!("The rich have lots of money!"),
    Poor => println!("The poort have no money..."),
  }

  match work {
    Civilian => println!("Civilians work!"),
    Soldier => println!("Soldiers fight!"),
  }

  let n=16;
  println!("This is {}",LANGUAGE);
  println!("The threshold is {}", THRESHOLD);
  println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

  let unit=();

  println!("unit: {:?}",unit);
}