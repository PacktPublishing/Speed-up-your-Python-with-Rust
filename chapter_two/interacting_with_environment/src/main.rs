mod stocks;

use stocks::{open_order, close_order};
use stocks::structs::order::Order;
use stocks::enums::order_types::OrderType;

use std::env;
use rand::prelude::*;
use std::str::FromStr;


fn main() {
    let args: Vec<String> = env::args().collect();
    let action: &String = &args[1];
    let name: &String = &args[2];
    let amount: i32 = i32::from_str(&args[3]).unwrap();
    let price: f32 = f32::from_str(&args[4]).unwrap();

    let mut new_order: Order = open_order(amount, OrderType::Long, 
                                         &name.as_str(), 
                                         price, 
                                      None, None);

    match action.as_str() {
        "buy" => {
            println!("the value of your investment is: {}", 
                     new_order.current_value());
        }
        "sell" => {
            let mut rng = rand::thread_rng();
            
            let new_price_ref: f32 = rng.gen(); 
            let new_price: f32 = new_price_ref * 100 as f32;

            new_order.stock.update_price(new_price);
            let sale_profit: f32 = close_order(new_order);

            println!("here is the profit you made: {}", sale_profit);
        }
        _ => {
            panic!("Only 'buy' and 'sell' actions are supported");
        }
    }
}
