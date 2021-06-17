mod stocks;

use stocks::{open_order, close_order};
use stocks::structs::order::Order;
use stocks::enums::order_types::OrderType;


fn main() {
    println!("hello stocks");
    let mut new_order: Order = open_order(20, OrderType::Long, "bumper", 56.8, 
                                      None, None);

    println!("the current price is: {}", &new_order.current_value());
    println!("the current profit is: {}", &new_order.current_profit());
    
    new_order.stock.update_price(43.1);
    println!("the current price is: {}", &new_order.current_value());
    println!("the current profit is: {}", &new_order.current_profit());

    new_order.stock.update_price(82.7);
    println!("the current price is: {}", &new_order.current_value());
    println!("the current profit is: {}", &new_order.current_profit());

    let profit: f32 = close_order(new_order);
    println!("we made {} profit", profit);
}
