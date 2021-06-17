pub mod structs;
pub mod enums;

use structs::stock::Stock;
use structs::order::Order;
use enums::order_types::OrderType;


/// Opens a stock order. 
/// 
/// # Arguments 
/// * number (132): the number of stock in the order
/// * order_type (OrderType): the type of order being made
/// * stock_name (&str): the name of the stock being ordered
/// * open_price (f32): the current price of the stock    
/// * stop_loss (Option<f32>): how much we are willing to lose before automatically selling 
/// * take_profit (Option<f32>): how much we are willing to gain before automatically selling
/// 
/// # Returns 
/// * (Order): the order that has been made  
pub fn open_order(number: i32, order_type: OrderType, stock_name: &str, open_price: f32,
                  stop_loss: Option<f32>, take_profit: Option<f32>) -> Order {
    println!("order for {} is being made", &stock_name);
    let mut stock: Stock = Stock::new(stock_name, open_price);
    match stop_loss {
        Some(value) => stock = stock.with_stop_loss(value),
        None => {}
    }
    match take_profit {
        Some(value) => stock = stock.with_take_profit(value),
        None => {}
    }
    return Order::new(stock, number, order_type)
}

/// Closes a stock order.
/// 
/// # Arguments
/// * order (Order): the order that is being closed
/// 
/// # Returns 
/// * (f32) the profit from that order
pub fn close_order(order: Order) -> f32 {
    println!("order for {} is being closed", &order.stock.name);
    return order.current_profit()
}