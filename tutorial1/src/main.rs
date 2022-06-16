fn main() {
    // Mutable variables, let mut x = 4;
    let x = 4;
    println!("x is : {}", x);
    let x = x + 1;
    println!("x is : {}", x);

    // Shadowing
    {
        let x =  x - 2;
        println!("Interior Scope variable");
        println!("x is: {}", x);
    }
    let x = x + 1;
    println!("x is : {}", x);

    let x = "hello";
    println!("x is: {}", x);

    // Constants
    const SECOND_IN_MINUTE: u32 = 60;
    const SECOND_IN_MINUTE: u32 = 100;
    println!("{}", SECOND_IN_MINUTE);
}
