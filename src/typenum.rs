fn main() {
    let x = 5;
    let y= 0xff;
    let z = 0b1111_1111;
    let _a = 0o77; 
    let v = 1024 + y + z + _a;
    println!("{}", v);
    println!("{}, {}", "i32".to_string(), type_of(&x));
    println!("{}, {}", i8::MAX, u8::MAX);
    println!("{}{}", y, z);
    let x = 1_000.000_1;
    println!("{}, {}", x, type_of(&x));
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}