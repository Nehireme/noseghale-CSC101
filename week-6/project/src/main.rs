use std::io;
fn main() {
    println!("I'm sure you're hungry, you big back!!");
    println!("MENU\nP= Pounded yam and Edinkaiko soup, #3200\nF= Freid rice and chicken, #3000\nA= Amala and Ewedu soup, #2500\nE= Eba and Egusi soup, #2000\nW= White rice and stew, #2500 ");
    println!("");
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("What would you like to order?\n Type the letters indicated on your menu.[if you don't then no food for you ig ] ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let order = input1.trim().to_lowercase();

    println!("Quantity of meal??");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let quantity:i32 = input2.trim().parse().expect("Not a valid number");

    println!("");

    let mut unit_price:i32 = 0;
     let price = unit_price*quantity;
    if order == "p"{
    unit_price = 3200;
    println!("Price is {}", unit_price);
    }
    else if order == "f"{
    unit_price = 3000;
    println!("Price is {}", unit_price);
    }
    else if order == "a" {
    unit_price = 2500;
    println!("Price is {}", unit_price);
    }
    else if order == "e" {
    unit_price = 2000;
    println!("Price is {}", unit_price);
    }
    else if order == "w" {
    unit_price = 2500;
    println!("Price is {}", unit_price);
    }
    else {
        println!("INVALID ORDER, NO FOOD FOR YOUUUUU")
    }
    let price = unit_price*quantity;
    println!("your total order is {}", price);

     if price >= 10000{
        println!("CONGRATS YOU GET A DISCOUNT FOR BEING A MONEY SPENDER");
        let new_price = price as f32*0.95;
        println!("your discounted price is {} ", new_price);
        println!("");
        println!("Thanks for coming\nEnjoy your food");
        println!("");
        println!("PLEASE DONT SHOW YOUR FACE HERE AGAIN, THIS REQUIRES CODING AND I DO NOT HAVE THE TALENT FOR THAT");

    }

}