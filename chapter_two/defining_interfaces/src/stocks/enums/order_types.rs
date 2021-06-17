

/// This enum is responsible for determining what type of order is being made.
/// 
/// # Fields 
/// * Short: this is when we borrow money from the broker to buy stock, we then sell
///          the stock with the intent to buy it back later, if the stock price 
///          drops we keep the difference making money 
/// * Long: this is when we buy the underlying asset with cash. If the stock price 
///         increases we make money when we sell
pub enum OrderType {
    Short,
    Long
}