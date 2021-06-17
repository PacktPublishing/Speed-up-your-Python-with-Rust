use chrono::{Local, DateTime};
use super::stock::Stock;
use super::super::enums::order_types::OrderType;


/// This struct is responsible for managing data around a stock order.Stock
/// 
/// # Fields 
/// * date (DateTime<Local>): the datetime when the order was made 
/// * stock (Stock): the stock being involved in the order 
/// * number (i32): the number of stock in the order
/// * order_type (OrderType): the type of order being made
pub struct Order {
    pub date: DateTime<Local>,
    pub stock: Stock,
    pub number: i32,
    pub order_type: OrderType
}

impl Order {
    /// The constructor for the Order struct.
    /// 
    /// # Argument
    /// * stock (Stock): the stock being involved in the order
    /// * number (i32): the number of stock in the order
    /// * order_type (OrderType): the type of order being made
    /// 
    /// # Returns
    /// * (Order): the created Order 
    pub fn new(stock: Stock, number: i32, order_type: OrderType) -> Order {
        let today: DateTime<Local> = Local::now();
        return Order{date: today, stock, number, order_type}
    }

    /// Calculates the current profit of the order. 
    /// 
    /// # Returns 
    /// * (f32): the profit of the order
    pub fn current_profit(&self) -> f32 {
        let current_price: f32 = self.current_value();
        let initial_price: f32 = self.stock.open_price * self.number as f32;

        match self.order_type {
            OrderType::Long => return current_price - initial_price,
            OrderType::Short => return initial_price - current_price
        }
    }

    /// Calculates the current value of the order. 
    /// 
    /// # Returns 
    /// * (f32): the current value of the order
    pub fn current_value(&self) -> f32 {
        return self.stock.current_price * self.number as f32
    }
}