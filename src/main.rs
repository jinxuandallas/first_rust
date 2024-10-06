// use std::path;
// use first_rust::test1;
use first_rust::test2;
// use test1::test3;
mod test1;

// use crate::lib::test1;
fn main() {
    println!("Hello, world!");

    let p=(1i32,2i32);
    let (a,b)=p;
    let x=p.0;
    let y=p.1;

    println!("{} {} {} {}", a, b, x, y);

    enum Animal{
        dog=1,
        cat=200,
        tiger,
    }

    let x=Animal::tiger as isize;
    println!("{}", x);

    let v=loop {
        break 10;
    };
    println!("{}", v);

    let v=Some(3u8);
    if let Some(4) =v  {
        println!("three");
    }
    
    println!("{:?}",v);

    // #[path ="lib.rs"]
    // mod  lib;
    // lib::test1();
    // test2();
    test1::test3();
}
