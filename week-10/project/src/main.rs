struct laptop{
    hp:u32, ibm:u32, toshiba:u32, dell:u32
}
fn main(){
    println!("ALABA INTERNATIONAL MARKET");
    println!("HP costs #650,000\nIBM costs #755,000\nToshiba costs #550,000\nDell costs #850,000");
    println!("You apparently bought 3 of each, idk why tho");

    let price = laptop {
        hp:650000*3,
        ibm:755000*3,
        toshiba:550000*3,
        dell:850000*3
    };
    let total = price.hp+price.ibm+price.toshiba+price.dell;
    println!("hp cost is {}, ibm cost is {}, toshiba cost is {}, dell cost is {}",price.hp, price.ibm, price.toshiba, price.dell);
    println!("total cost is {}",total);
}