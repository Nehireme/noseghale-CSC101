fn main() {
    let num:i32 = 5;
    mutate(num);
    println!("The value of no is: {}", num);
}
fn mutate(mut param_num: i32) {
    param_num = param_num*0;
    println!("param_num value is: {}", param_num);
}