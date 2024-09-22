fn main() {
    println!("Hello, world!");

    let p=(1i32,2i32);
    let (a,b)=p;
    let x=p.0;
    let y=p.1;

    println!("{} {} {} {}", a, b, x, y);
}
