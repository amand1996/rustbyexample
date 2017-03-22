#[derive(Debug)]
struct Structure(i32);
#[derive(Debug)]
struct Deep(Structure);

use std::fmt;
#[derive(Debug)]
struct Complex{
    real : f64,
    imag : f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator, or try!, to return on errors.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}:{}", count, v)?;
        }
        write!(f, "]")
    }
}

fn main() {
    println!("Hello, world!");
    let x = 5 + /* 40 + */ 5;
    println!("Is x 10 or 50? x = {}", x);
    println!("{} days", 31);
    println!("{0} is {1}'s friend!","Aman", "Sachin");
    println!("{a} is {b} {c} works!", a = "This", b = "how", c = "Rust");
    println!("{:b} is binary of {}", 2, 2);
    let pi = 3.141592;
    let formatted_pi = format!("{:.*}", 3, pi);
    println!("pi = {}", formatted_pi);

    println!("{:?} months in a year.", 12);
    println!("Now {:?} will print!", Structure(3));

    println!("Now {:?} will print!", Deep(Structure(7)));
    let complex = Complex{real : 3.3, imag : 7.2};
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);

    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
