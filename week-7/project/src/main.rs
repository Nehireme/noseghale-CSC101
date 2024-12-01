use std::io;

fn main() {

    println!("YOU DO  NOT KNOW MATH, LET ME HELP :)\n\nWhat shape's formula do you need now ?\n\nArea of Trapezium = 1\nArea of Rhombus = 2\nArea of Parallelogram = 3\nArea of Cube = 4\nVolume of Cylinder = 5");

let mut input1 = String::new();
    println!("Please type the corresponding number");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let shape:u32 = input1.trim().parse().expect("Not valid");
    println!("You chose {}", shape);

    if shape == 1{
    trapezium()
    }
    else if shape == 2{
        rhombus()
    }
    else if shape == 3{
        parallelogram()
    }
    else if shape == 4{
        cube()
    }
    else if shape == 5{
        cylinder()
    }

    fn trapezium() {

        println!("You chose trapezuim\nTime to find the area");
        println!("Enter base1");
        let mut b1 = String::new();
        io::stdin().read_line(&mut b1).expect("invalid");
        let base1:f32 = b1.trim().parse().expect("stuff");

        println!("Enter base2");
        let mut b2 = String::new();
        io::stdin().read_line(&mut b2).expect("invalid");
        let base2:f32 = b2.trim().parse().expect("stuff");

        println!("Enter height");
        let mut heightt= String::new();
        io::stdin().read_line(&mut heightt).expect("invalid");
        let height_of_trapezuim:f32 = heightt.trim().parse().expect("stuff");

        let areaT = height_of_trapezuim/2.0*(base1+base2);
        println!("Area of Trapezuim is {}", areaT);
    }
    fn cube() {
        println!("You chose cube\nTime to finnd the area");
        println!("Enter Length of side");
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("invalid");
        let side:f32 = s.trim().parse().expect("stuff");

        let areaC = 6.0*(side).powf(2.0);
        println!("Area of Cube is {}", areaC);
    }

    fn cylinder() {
         println!("You chose cylinder\nTime to find the volume");
        println!("Enter radius");
        let mut r = String::new();
        io::stdin().read_line(&mut r).expect("invalid");
        let radius:f32 = r.trim().parse().expect("stuff");

        println!("Enter Height");
        let mut heightv = String::new();
        io::stdin().read_line(&mut heightv).expect("invalid");
        let height_of_cylinder:f32 = heightv.trim().parse().expect("stuff");

        let volume = (22.0/7.0)*radius.powf(2.0)*height_of_cylinder; 
        println!("Volume of cylinnder is {}", volume);
    }

    fn rhombus() {
         println!("You chose rhombus\nTime to find the area");
        println!("Enter diagonal1");
        let mut d1 = String::new();
        io::stdin().read_line(&mut d1).expect("invalid");
        let diagonal1:f32 = d1.trim().parse().expect("stuff");

        println!("Enter diagonal2");
        let mut d2 = String::new();
        io::stdin().read_line(&mut d2).expect("invalid");
        let diagonal2:f32 = d2.trim().parse().expect("stuff");

        let areaR = (1.0/2.0)*diagonal1*diagonal2;
        println!("Area of Rhombus is {}", areaR);
    }
    fn parallelogram() {
         println!("You chose parallelogram\nTime to find the area");
        println!("Enter base");
        let mut b = String::new();
        io::stdin().read_line(&mut b).expect("invalid");
        let base:f32 = b.trim().parse().expect("stuff");

        println!("Enter base2");
        let mut a = String::new();
        io::stdin().read_line(&mut a).expect("invalid");
        let altitude:f32 = a.trim().parse().expect("stuff");

        let areaP = base*altitude;
        println!("Area of Parallelogram is {}", areaP);
    }
}
