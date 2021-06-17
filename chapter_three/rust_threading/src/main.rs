use std::{thread, time};
use std::thread::JoinHandle;

use std::boxed::Box;
use std::any::Any;
use std::marker::Send;


fn simple_thread(seconds: i8, name: &str) -> i8 {
    println!("thread {} is running", name);
    let total_seconds = time::Duration::new(seconds as u64, 0);
    thread::sleep(total_seconds);
    println!("thread {} has finished", name);
    return seconds
}


fn process_thread(thread_result: Result<i8, Box<dyn Any + Send>>, name: &str) {
    match thread_result {
        Ok(result) => {
            println!("the result for {} is {}", result, name);
        }
        Err(result) => {
            if let Some(string) = result.downcast_ref::<String>() {
                println!("the error for {} is: {}", name, string);
            } else {
                println!("there error for {} does not have a message", name);
            }
        }
    }
}


fn main() {
    let now = time::Instant::now();

    let thread_one: JoinHandle<i8> = thread::spawn(|| {
                                     simple_thread(5, "one")});
    let thread_two: JoinHandle<i8> = thread::spawn(|| {
                                     simple_thread(5, "two")});
    let thread_three: JoinHandle<i8> = thread::spawn(|| {
                                       simple_thread(5, "three")});

    let result_one: Result<i8, Box<dyn Any + Send>> = thread_one.join();
    let result_two: Result<i8, Box<dyn Any + Send>> = thread_two.join();
    let result_three: Result<i8, Box<dyn Any + Send>> = thread_three.join();

    println!("time elapsed {:?}", now.elapsed());

    process_thread(result_one, "one");
    process_thread(result_two, "two");
    process_thread(result_three, "three");
}