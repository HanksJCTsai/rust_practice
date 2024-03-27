fn main() {
    //Declare immutable variable
    let _nice_count = 100;
    let _nice_number: i64 = 54;
    //_nice_count = 23;

    //Declare mutable variable
    let mut _count = 3;
    _count = 4;

    // Shadowing
    let _x:i32 = 5;{
        //namespace 
        let _x:i64 = 10;
        println!("Inner x: {}", _x);
    }// Inner _x is destroyed
    println!("Outer x: {_x}");
    // Declare a variable with string type annotation
    let _x : &str = "Hello"; //on the scope of the variable, recovers the value of the variable
    println!("New _x: {}", _x);
    //_x = "World";
    let mut _x:&str = "World";
    println!("_x: {}", _x);
    _x = "Hello_2";
    println!("Mutable variable _x: {}", _x);
}
