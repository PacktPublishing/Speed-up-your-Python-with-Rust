mod stocks;

use stocks::structs::stock::Stock;


fn main() {
    let stock: Stock = Stock::new("MonolithAi", 36.5);
    println!("here is the stock name: {}", stock.name);
    println!("here is the stock price: {}", stock.current_price);
}
