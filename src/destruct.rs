fn main() {
    let (x, y);
    (x, ..) = (2, 4);
    (.., y) = (1, 2);
    println!("{}, {:?}", x, y); 

    
    let triple = (1, -5, 6);
    println!("triple = {:?}", triple);
    
    match triple {
        (3, y, z) => println!("#1 = 3, y = {:?}, z = {:?}", y, z),
        (1, ..) => println!("#1 = {}, others does not matter", x),
        (2, .., 4) => println!("#1 2 and #3 4"),
        (.., z) => println!("#1s do not matter, #3 = {}", z),
    };
}