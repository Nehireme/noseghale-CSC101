use std::io;
fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter value of a :");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:i32 = input1.trim().parse().expect("Not valid number");

    println!("Enter value of b:");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:i32 = input2.trim().parse().expect("Not a alid nummber");

    println!("Enter value of c: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:i32 = input3.trim().parse().expect("Not a valid umber");

    let discriminant = (b.pow(2)) - 4*a*c; 
    println!("Discriminant is {}", discriminant);

    if discriminant > 0 {
        println!("There are two  distinct roots");
    } else if discriminant == 0 {
        println!("There is one root");
    } else if discriminant<0 {
        println!("There are no real roots");
    } 
    let root1 = (-b.pow(2) + ((b - 4*a*c).pow(1/2))) / 2*a;
    let root2 = (-b.pow(2) - ((b - 4*a*c).pow(1/2))) / 2*a;

println!("Roots of the equation are {} and {}", root1, root2);

}