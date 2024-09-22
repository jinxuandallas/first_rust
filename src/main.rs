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
}
