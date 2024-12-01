fn main() {
    let mut num:i32 = 5;
    mutate(&mut num);
    println!("The value of num is: {}",num); 
}
fn mutate(param:&mut i32){
    *param = *param*0;
    println!("param value is: {}",param);
}