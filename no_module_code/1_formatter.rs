use std::fmt; // Import `fmt`

#[derive(Debug)] struct MinMax(i64, i64);

impl fmt::Display for MinMax {
  // fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
  //   write!(f, "(.0: {}, .1:{})", self.0, self.1)
  // }
  fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
    write!(f, "(.0: {}, ", self.0)?; // write to buffer
    write!(f, "___")?; // write to buffer
    let _k=8; // put other code between ? is possible
    write!(f, " ,.1:{})", self.1)
  }
}

struct Color{
  name: &'static str,
  red: u8,
  green: u8,
  blue: u8,
}

impl fmt::Display for Color {
  fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} ", self.name)?;
    write!(f, " RGB({},{},{})",self.red,self.green,self.blue)?;
    
    let x_r = format!("{:X}",self.red);
    let x_g = format!("{:X}",self.green);
    let x_b = format!("{:X}",self.blue);
    write!(f, " 0x{0:0>2}{1:0>2}{2:0>2}",x_r, x_g, x_b)
  }
}

fn main() {
  let minmax = MinMax(0, 9);
  println!("Compare structures:");
  println!("Display: {}",minmax);
  println!("Debug: {:?}",minmax);

  let teal = Color{
    name: "Teal",
    red: 0,
    green: 128,
    blue: 128
  };
  println!("{0}",teal);
}