pub fn calculate_price(amount: u32) -> u32 {
    let mut price = 2;
    if amount >= 40 {
        price = 1;
    }
    
    price * amount
}
