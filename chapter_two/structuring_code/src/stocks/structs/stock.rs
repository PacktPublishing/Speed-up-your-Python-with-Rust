use super::utils::constructor_shout;


/// This struct is responsible for managing the data around a stock.
/// 
/// # Fields 
/// * name (String): the name of the stock
/// * open_price (f32): the price when the stock was initially brought
/// * stop_loss (f32): (Optional) how much we are willing to lose before automatically selling
/// * take_profit (f32): (Optional) how much we are willing to gain before automatically selling   
/// * current_price (f32): the current price of the stock                
pub struct Stock {
    pub name: String,
    pub open_price: f32,
    pub stop_loss: f32,
    pub take_profit: f32,
    pub current_price: f32
}

impl Stock {
    /// The constructor for the Stock struct. 
    /// 
    /// # Arguments 
    /// * stock_name (&str): the name of the stock being created
    /// * price (f32): the price of the stock being created
    /// 
    /// # Returns 
    /// * (Stock): the created stock
    pub fn new(stock_name: &str, price: f32) -> Stock {
        constructor_shout(stock_name);
        return Stock{
            name: String::from(stock_name), 
            open_price: price,
            stop_loss: 0.0,
            take_profit: 0.0,
            current_price: price
        }
    }
    /// Adds a ```stop_loss``` field to the Stock struct 
    /// 
    /// # Arguments 
    /// * value (f32): how much we are willing to lose before automatically selling
    /// 
    /// # Returns 
    /// * (Stock): stock with the ```stop_loss``` field
    pub fn with_stop_loss(mut self, value: f32) -> Stock {
        self.stop_loss = value;
        return self
    }
    /// Adds a ```take_profit``` field to the Stock struct 
    /// 
    /// # Arguments 
    /// * value (f32): how much we are willing to gain before automatically selling
    /// 
    /// # Returns 
    /// * (Stock): stock with the ```take_profit``` field
    pub fn with_take_profit(mut self, value: f32) -> Stock {
        self.take_profit = value;
        return self
    }
    /// Updates the ```current_price``` field of the struct. 
    /// 
    /// # Arguments 
    /// * value (f32): the new price of the stock
    pub fn update_price(&mut self, value: f32) {
        self.current_price = value;
    }
}

