fn main() {
    // let x: i32 = 5;
    // let y: i32 = 4;

    // assert_eq!(x, 5);
    // assert_eq!(y, 4);
    // println!("Success");

    // let mut x = 1;
    // x += 2;

    // assert_eq!(x, 3);
    // println!("Success");

    // let x = 5;
    // {
    //     let y = 5;
    //     let x = 6;
    //     println!("Inner scope value of x is {} and value of y is {}", x, y);
    // }

    // println!("Outer scope value of x is {}", x);

    // let mut x: i32 = 1;
    // x = 7;
    // // Shadowing and re-binding
    // // let x = x; 
    // x += 3;


    // let y = 4;
    // // Shadowing
    // let y = "I can also be bound to text!"; 

    // println!("Success!");

    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}
