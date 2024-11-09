use std::io;

fn main(){
    let mut input = String::new();

    println!("\nEnter Your Height (in centimetres):");
    io::stdin().read_line(&mut input).expect("Not a valid stirng");
    let height:f32 = input.trim().parse().expect("not a valid number");

    if height>150.0 && height <170.0
    {
        println!("You are an average person");
    }
    else if height>170.0 && height <195.0
    {
        println!("You tallll dangg");
    }
    else if height>150.0 && height <100.0
    {
        println!("You short asf");
    }
    else {
        println!("ABNORMAL");
    }
}