use std::collections::HashMap;


enum Value {
    Str(&'static str),
    Int(i32),
}


fn check_int_above_threshold(threshold: i32, get_result: Option<&Value>) -> Result<bool, &'static str> {
    match get_result {
        Some(inside_value) => {

            match inside_value {
                Value::Str(_) => return Err("str value was supplied as opposed to an int which is needed"),
                Value::Int(int_value) => {
                    if int_value > &threshold {
                        return Ok(true)
                    }
                    return Ok(false)
                } 
            }
        }
        None => return Err("no value was supplied to be checked")
    }
}


fn main() {
    let mut map = HashMap::new();
    map.insert("one", Value::Str("1"));
    map.insert("two", Value::Int(2));

    let result: Option<&Value> = map.get("two");
    let above_threshold: bool = check_int_above_threshold(1, result).unwrap();
    println!("it is {} that the threshold is breached", above_threshold);

    let second_result: Option<&Value> = map.get("one");
    let second_threshold: bool = check_int_above_threshold(1, second_result).expect("an error happened");
}