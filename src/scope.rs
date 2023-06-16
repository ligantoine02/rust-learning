fn main() {
    let x: u32 = 5;
    {
        let y: u32 = 8;
        println!("Inside variables, x = {} & y = {}", x, y);
    }
    println!("Outside variables, x= {}", x);
    define_x();
}

fn define_x() {
    let x = "hello";
    println!("{}, world", x);
}